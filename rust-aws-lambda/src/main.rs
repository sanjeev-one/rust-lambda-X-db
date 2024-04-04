use lambda_runtime::{service_fn, Context, Error, LambdaEvent};
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput, ScanInput};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error as StdError;
use tracing::{error, info, Level};
use tracing_subscriber;

#[derive(Deserialize)]
struct Request {
    command: String,
}

#[derive(Serialize, Debug)]
struct Response {
    req_id: String,
    msg: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .without_time()
        .init();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

async fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let command = event.payload.command.clone();
    let request_id = event.context.request_id.clone();

    info!(
        "Received command: {} with request ID: {}",
        command, request_id
    );

    let mut response_message = format!("Command '{}' executed!", command);

    if command == "print_db" {
        match print_db_contents().await {
            Ok(db_contents) => {
                response_message = format!("{} DB Contents: {}", response_message, db_contents);
                info!("Database contents fetched successfully.");
            }
            Err(e) => {
                error!("Error printing DB contents: {}", e);
                return Err(lambda_runtime::Error::from(format!(
                    "Failed to print DB contents: {}",
                    e
                )));
            }
        }
    }

    // Log the command to DynamoDB
    if let Err(e) = log_command(command.clone(), request_id.clone()).await {
        error!("Failed to log command: {} due to error: {}", command, e);
        return Err(lambda_runtime::Error::from(e.to_string()));
    }

    let resp = Response {
        req_id: request_id,
        msg: response_message,
    };

    info!("Response prepared: {:?}", resp);

    Ok(resp)
}

// Include your existing log_command and print_db_contents functions without modification here.

// Utility function to print contents of the DynamoDB table
// Note: This should be adjusted according to where and how you intend to call it
async fn print_db_contents() -> Result<String, Box<dyn StdError + Send + Sync>> {
    let client = DynamoDbClient::new(Region::default());
    let scan_request = ScanInput {
        table_name: "my-table".to_string(),
        ..Default::default()
    };

    let result = client.scan(scan_request).await?;
    // Convert the result into a string
    let items_str = match result.items {
        Some(items) => format!("{:?}", items),
        None => "No items found".to_string(),
    };

    Ok(items_str)
}

async fn log_command(
    command: String,
    request_id: String,
) -> Result<(), Box<dyn StdError + Send + Sync>> {
    let client = DynamoDbClient::new(Region::default());
    let mut item = HashMap::new();
    item.insert(
        "request_id".to_string(),
        AttributeValue {
            s: Some(request_id),
            ..Default::default()
        },
    );
    item.insert(
        "command".to_string(),
        AttributeValue {
            s: Some(command),
            ..Default::default()
        },
    );

    let put_request = PutItemInput {
        table_name: "my-table".to_string(),
        item,
        ..Default::default()
    };

    client.put_item(put_request).await?;
    Ok(())
}
