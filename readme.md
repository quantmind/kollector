# Collect & stream orderbooks

[![build](https://github.com/quantmind/kollector/actions/workflows/build.yml/badge.svg)](https://github.com/quantmind/kollector/actions/workflows/build.yml)

A small service for collecting and streaming orderbook L2 data from crypto exchanges.

## Development

* you need rust toolchain installed (this has been developed with rust 1.6)
* run `make build` to build and test the rust application
* [protobuf definitions](./service/proto/orderbook.proto)
* code documentation https://quantmind.github.io/kollector/common/
* `yarn watch` will start the web server for development (serving on http://localhost:3000)


## Running the App

You can run the e2e app using docker rather than building from source.
To run server and a web client, make sure you have `docker-compose` installed and launch `make start`.

The command will start:

* a [kong](https://github.com/Kong/kong) gateway server configured for [grpc-web](https://docs.konghq.com/hub/kong-inc/grpc-web/)
* the rust `kollector` service
* the web server serving the front-end application on http://localhost:3000
