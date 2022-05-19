# Collect & stream orderbooks

[![build](https://github.com/quantmind/kollector/actions/workflows/build.yml/badge.svg)](https://github.com/quantmind/kollector/actions/workflows/build.yml)

A small service for collecting and streaming orderbook L2 data from crypto exchanges.

## Development

* you need rust toolchain installed (this has been developed with rust 1.6)
* run `make build` to build and test the application
* code documentation https://quantmind.github.io/kollector/common/


## Running the App

* make sure you have `docker-compose` installed
* `make start` will start
  * a [kong](https://github.com/Kong/kong) server configured for [grpc-web](https://docs.konghq.com/hub/kong-inc/grpc-web/)
  * the `kollector` service
* `yarn watch` will start the web server for testing
* navigate to http://localhost:3000 to see the streaming results
