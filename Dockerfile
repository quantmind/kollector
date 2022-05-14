FROM rust:1.60 AS builder

ARG BUILD_FLAG=

RUN apt update
RUN apt install -y protobuf-compiler
RUN protoc --version

WORKDIR /kollector

COPY . .

RUN cargo build $BUILD_FLAG
RUN cargo test $BUILD_FLAG

FROM debian:bullseye-slim

RUN apt update
RUN apt install -y openssl ca-certificates

ARG BIN=/usr/local/bin
ARG RELEASE=debug

COPY --from=builder /kollector/target/$RELEASE/service ${BIN}/kollector
RUN kollector --help
