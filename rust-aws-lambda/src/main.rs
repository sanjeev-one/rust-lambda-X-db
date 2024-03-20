use lambda_runtime::{service_fn, Error, LambdaEvent};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, AttributeValue, ScanInput};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error as StdError;

#[derive(Deserialize)]
struct Request {
    command: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError + Send + Sync>> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).without_time().init();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;

    // Example call to print_db_contents (might want to do this based on specific commands)
    // Note: In a real scenario, you might want to call this under specific conditions or in a different part of your application
    // print_db_contents().await?;

    Ok(())
}

async fn log_command(command: String, request_id: String) -> Result<(), Box<dyn StdError + Send + Sync>> {
    let client = DynamoDbClient::new(Region::default());
    let mut item = HashMap::new();
    item.insert("request_id".to_string(), AttributeValue { s: Some(request_id), ..Default::default() });
    item.insert("command".to_string(), AttributeValue { s: Some(command), ..Default::default() });

    let put_request = PutItemInput {
        table_name: "my-table".to_string(),
        item,
        ..Default::default()
    };

    client.put_item(put_request).await?;
    Ok(())
}

async fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let mut command = event.payload.command.clone();
    let request_id = event.context.request_id.clone();
    // Decide when to call print_db_contents based on the command or other conditions
    if command == "print_db" { // Example condition: check if the command is to print DB contents
        command += r#"db printed"#; 
        // Call print_db_contents and await its completion
        if let Err(e) = print_db_contents().await {
            // Handle error if print_db_contents fails, possibly logging the error or returning an error response
            eprintln!("Error printing DB contents: {}", e); // Log the error
            // You might choose to return an error response or continue execution
        }
    }

    // Log the command to DynamoDB (assumes this should happen for all commands, adjust as needed)
    if let Err(e) = log_command(command.clone(), request_id.clone()).await {
        return Err(lambda_runtime::Error::from(e.to_string()));
    }

    // Response to return after handling the command
    let resp = Response {
        req_id: request_id,
        msg: format!("Command '{}' executed!", command),
    };

    Ok(resp)
}

// Utility function to print contents of the DynamoDB table
// Note: This should be adjusted according to where and how you intend to call it
async fn print_db_contents() -> Result<(), Box<dyn StdError + Send + Sync>> {
    let client = DynamoDbClient::new(Region::default());

    let scan_request = ScanInput {
        table_name: "my-table".to_string(),
        ..Default::default()
    };

    let result = client.scan(scan_request).await?;
    println!("{:?}", result.items);

    Ok(())
}
