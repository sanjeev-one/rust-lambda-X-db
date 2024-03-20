// This example requires the following input to succeed:
// { "command": "do something" }

use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};



use aws_sdk_dynamodb::{Client, Config, Region};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_config::meta::region::RegionProviderChain;


use lambda_runtime::{Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use aws_sdk_dynamodb::{Client, Error as DynamoError};
use std::collections::HashMap;

#[derive(Deserialize)]
struct Request {
    command: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

async fn my_handler(
    event: LambdaEvent<Request>,
    dynamo_db_client: &Client,
) -> Result<Response, Error> {
    let request = event.payload;
    let request_id = event.context.request_id.clone();

    // Insert command into DynamoDB
    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S(uuid::Uuid::new_v4().to_string()));
    item.insert("command".to_string(), AttributeValue::S(request.command.clone()));

    dynamo_db_client.put_item()
        .table_name("CommandsTable")
        .set_item(Some(item))
        .send()
        .await
        .map_err(|e| Error::new(e.to_string()))?;

    // For simplicity, the count logic is not demonstrated here. You might update a specific item's attribute for the count, or use another mechanism.
    
    // Prepare the response
    Ok(Response {
        req_id: request_id,
        msg: format!("Command '{}' executed and stored.", request.command),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set up logging
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).without_time().init();

    // Initialize AWS SDK configuration
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let dynamo_db_client = Client::new(&config);

    // Set up your Lambda function handler
    let func = service_fn(|event| my_handler(event, dynamo_db_client.clone()));

    lambda_runtime::run(func).await?;

    Ok(())
}

