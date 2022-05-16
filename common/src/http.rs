use reqwest::{Client, Method, RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_urlencoded;
use slog;
use std::fmt;
use url::Url;

/// An HTTP Request
pub trait Request: Serialize + Send {
    const METHOD: Method;
    const PATH: &'static str;
    const HAS_PAYLOAD: bool = true;
    type Response: DeserializeOwned;
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum ErrorKind {
    Network,
    InvalidContentMatch,
    BadContent,
    BadStatus,
}

#[derive(Debug)]
pub struct Error {
    method: Method,
    url: Url,
    status: String,
    err: String,
    content: String,
    error_kind: ErrorKind,
}

type Result<T> = std::result::Result<T, Error>;

/// An Http client for Rest APIs
pub struct HttpClient {
    base_url: String,
    client: Client,
}

impl HttpClient {
    /// Create a new HttpClient with a base url
    pub fn new(url: &str) -> Self {
        Self {
            base_url: url.to_owned(),
            client: Client::new(),
        }
    }

    /// full url given a path
    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// create an unsigned request builder
    pub fn unsigned<R: Request>(&self, _req: &R, url: &Url, body: &str) -> RequestBuilder {
        let request = self.client.request(R::METHOD, url.clone());
        match body.is_empty() {
            true => request,
            false => request.header("content-type", "application/json"),
        }
    }

    /// perform the HTTP request
    pub async fn request<R>(&self, req: R, logger: Option<&slog::Logger>) -> Result<R::Response>
    where
        R: Request,
        R::Response: DeserializeOwned,
    {
        let url = self.url(R::PATH);
        let mut url = Url::parse(&url).expect("failed to parse url");

        if matches!(R::METHOD, Method::GET | Method::DELETE) && R::HAS_PAYLOAD {
            url.set_query(Some(
                &serde_urlencoded::to_string(&req).expect("failed to encode url payload"),
            ));
        }

        let body = match R::METHOD {
            Method::PUT | Method::POST => {
                serde_json::to_string(&req).expect("failed to json encode body payload")
            }
            _ => "".to_string(),
        };

        let request = self.unsigned(&req, &url, &body);

        // do some logging if logger provided
        if let Some(log) = logger {
            slog::debug!(log, "HttpClient {} {}", R::METHOD, url);
        }

        let response = request
            .header("user-agent", "quantmind-trading")
            .body(body)
            .send()
            .await
            .unwrap();

        self.handle_response::<R::Response>(&R::METHOD, &url, response)
            .await
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        method: &Method,
        url: &Url,
        resp: Response,
    ) -> Result<T> {
        let status = resp.status();
        match resp.text().await {
            Ok(content) => {
                if status.is_success() {
                    serde_json::from_str::<T>(&content).or_else(|err| {
                        Err(Error::new(
                            method.clone(),
                            url.clone(),
                            status,
                            ErrorKind::InvalidContentMatch,
                            err.to_string(),
                            content,
                        ))
                    })
                } else {
                    Err(Error::new(
                        method.clone(),
                        url.clone(),
                        status,
                        ErrorKind::BadStatus,
                        "".to_string(),
                        content,
                    ))
                }
            }
            Err(err) => Err(Error::new(
                method.clone(),
                url.clone(),
                status,
                ErrorKind::BadContent,
                err.to_string(),
                "".to_string(),
            )),
        }
    }
}

fn trim(text: String, len: usize) -> String {
    let text_len = text.len();
    match text_len > len + 3 {
        true => {
            let mut msg = text;
            msg.truncate(len);
            msg.push_str(&format!("...({} more characters)", text_len - len));
            msg
        }
        false => text,
    }
}

impl Error {
    pub fn new(
        method: Method,
        url: Url,
        status: StatusCode,
        error_kind: ErrorKind,
        err: String,
        content: String,
    ) -> Self {
        Error {
            method,
            url,
            status: status.as_str().to_string(),
            err,
            error_kind,
            content: trim(content, 1000),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.error_kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fluid::http::Error {} {} - response status {}\n{:?} {}\n{}",
            self.method, self.url, self.status, self.error_kind, self.err, self.content
        )
    }
}
