# import local .env with local env variables
$(shell touch .env)
include .env
export $(shell sed 's/=.*//' .env)
RUST_VERSION = 1.60

.PHONY: help build

help:
	@echo ================================================================================
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
	@echo ================================================================================


build:			## build and test
	cargo build
	cargo test

cloc:			## Count lines of code - requires cloc
	cloc --exclude-dir=target .

doc-ci:			## build documentation
	@cargo doc --workspace --no-deps

doc:			## build documentation and open web page
	@cargo doc --workspace --no-deps --open

docker-login:		## login to docker repos - this is for admins only
	@./devops/docker-login

image:			## build docker image
	docker build . -t kollector

image-push:		## push image to repo
	@echo skip

lint:			## format code
	@cargo fmt
	@cargo clippy --fix

test:			## run tests
	@echo skip

test-lint:		## lint
	@cargo fmt --check
