# AWS Lambda DynamoDB Logger

This Rust project defines an AWS Lambda function designed to log commands received through events into an Amazon DynamoDB table and, optionally, fetch and display the contents of this table. The Lambda function is built using `lambda_runtime` for the Rust runtime and `rusoto_dynamodb` for interacting with DynamoDB.

## Features

- **Log Commands:** Logs each command received through a Lambda event to a DynamoDB table.
- **Print Database Contents:** On receiving a specific command (`print_db`), the Lambda function scans the specified DynamoDB table and returns its contents as part of the response message.

## Requirements

- Rust and Cargo
- AWS CLI configured with appropriate access rights
- An existing Amazon DynamoDB table

## Setup

### DynamoDB Table

Ensure you have a DynamoDB table created with the name `my-table` and a primary key named `request_id`.

### Deployment Package

Build your Lambda function package by compiling the Rust project:

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

Then, create a deployment package by zipping the compiled binary:

```bash
zip -j rust_lambda.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
```

### Lambda Function

Create an AWS Lambda function specifying the runtime as provided and upload the `rust_lambda.zip` as the function code. Ensure the Lambda function's execution role has permissions to access DynamoDB (at least `dynamodb:PutItem` and `dynamodb:Scan` on your table).

## Usage

Invoke the Lambda function by sending an event with a `command` field. The `command` can be any string, but if `print_db` is sent, the function will scan the DynamoDB table and return its contents.

### Example Event

```json
{
  "command": "example_command"
}
```

### Special Command

To print the contents of the DynamoDB table, use:

```json
{
  "command": "print_db"
}
```

## Development

### Local Setup

Ensure you have Rust and Cargo installed. Clone the repository and navigate into the project directory. Then use cargo lambda to deploy and invoke.

