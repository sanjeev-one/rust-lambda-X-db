install:
	#unfortunately python dependencies needed
	pip3 install cargo-lambda
format:
	cargo fmt 

lint:
	cargo clippy 

test:
	cargo test

run:
	cargo run 

release:
	cargo lambda build --release

release-arm:
	cargo lambda build --release --arm64

deploy:
	cargo lambda deploy

invoke:
	cargo lambda invoke --remote \
  		--data-ascii '{"command": "hello tihs is a test can xray see this"}' \
  		--output-format json \
  		rust-aws-lambda
print:
	cargo lambda invoke --remote \
  		--data-ascii '{"command": "print_db"}' \
  		--output-format json \
  		rust-aws-lambda

watch:
	cargo lambda watch		

all: format lint test run