# import local .env with local env variables
$(shell touch .env)
include .env
export $(shell sed 's/=.*//' .env)
RUST_VERSION = 1.60

.PHONY: help build cloc doc-ci doc docker-login image image-push web lint test test-lint

help:
	@echo ================================================================================
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
	@echo ================================================================================


build:			## build and test
	cargo build
	cargo test

cloc:			## Count lines of code - requires cloc
	cloc --exclude-dir=target,.venv,node_modules,dist,.mypy_cache .

doc-ci:			## build documentation
	@cargo doc --workspace --no-deps

doc:			## build documentation and open web page
	@cargo doc --workspace --no-deps --open

docker-login:		## login to docker repos - this is for admins only
	@./devops/docker-login

image:			## build docker image
	docker build . -f devops/kollector.dockerfile -t kollector

image-web:		## build docker image
	docker build . -f devops/web.dockerfile -t kollector-web

image-push:		## push image to repo
	@echo skip

web:			## build web interface
	protoc -I ./service/proto orderbook.proto --js_out=import_style=commonjs:web/proto
	protoc -I ./service/proto orderbook.proto --grpc-web_out=import_style=commonjs,mode=grpcwebtext:web/proto

lint:			## lint code
	@./devops/lint-py
	@cargo fmt
	@cargo clippy

lint-py:
	@./devops/lint-py

start:			## start dev services
	@docker-compose  -f devops/docker-compose.yml up

test:			## run tests
	@echo skip

test-lint:		## lint
	@cargo fmt --check
	@./devops/lint-py --check
