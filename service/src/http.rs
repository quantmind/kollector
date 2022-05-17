use common::{wrap_result, WorkerContext};
extern crate hyper;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header::CONTENT_TYPE, Body, Method, Request, Response, Server, StatusCode};
use prometheus::{Encoder, TextEncoder};
use std::net::SocketAddr;
extern crate anyhow;
use slog::info;

async fn http_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/status") => Ok(Response::new("OK".into())),
        (&Method::GET, "/metrics") => serve_prometheus_req(req).await,
        _ => not_found(req),
    }
}

pub fn not_found(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

pub async fn serve_prometheus_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let encoder = TextEncoder::new();

    let metric_families = prometheus::gather();

    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();

    Ok(response)
}

/// Starts an HTTP service
///
/// This service expose a liveness probe and prometheus metrics.
/// The port is configured via the `app_http_port` environment variable and defaults to 8050.
pub async fn start_http_service(context: WorkerContext) {
    let port: u16 = context.get_or("app_http_port", 8050).expect("HTTP port");
    // create the redis client
    // let _redis_cli = context.redis_cli("REDIS_URL").unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let make_service = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, hyper::Error>(service_fn(http_handler))
    });
    let server = Server::bind(&addr).serve(make_service);
    info!(context.logger, "start http server on {}", addr);
    let result = server.await.map_err(anyhow::Error::new);
    wrap_result(&context, result).await;
}
