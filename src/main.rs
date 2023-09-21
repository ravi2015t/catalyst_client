use reqwest;
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Create a vector to store the tasks for making requests
    let mut tasks = Vec::new();

    // Define the URL you want to request (replace with your actual URL)
    let url = "http://127.0.0.1:8000/calculate";

    // Define the number of parallel requests
    let num_requests = 100;

    for i in 1..num_requests {
        // Create a task for making a request and add it to the tasks vector
        tasks.push(tokio::spawn(make_request(url, i)));
    }

    // Wait for all tasks to complete
    let start_time = std::time::Instant::now();
    for task in tasks {
        task.await.expect("Task failed");
    }
    let end_time = std::time::Instant::now();

    // Calculate the total time taken
    let duration = end_time - start_time;

    println!("Total time taken: {:?}", duration);

    Ok(())
}

async fn make_request(url: &str, id: u16) -> Result<(), reqwest::Error> {
    // Create a Reqwest client
    let client = reqwest::Client::new();
    // Define the input data as a map of key-value pairs with dynamic types
    let mut input_data = HashMap::new();
    input_data.insert("bool_value".to_string(), json!(true));
    input_data.insert("int_value".to_string(), json!(42));
    input_data.insert("float_value".to_string(), json!(3.14));
    input_data.insert("string_value".to_string(), json!("Hello, World!"));

    // Create a JSON object from the input data
    let json_data: Value = json!({
        "data": input_data,
        "id":id,
    });

    // Send a POST request to the Actix web server
    let response = client
        .post(url)
        .json(&json_data)
        .send()
        .await
        .expect("Failed to send calculate call");

    // let resp_health_check = client
    //     .get("http://127.0.0.1:8000/health_check")
    //     .send()
    //     .await
    //     .expect("Health check failed");

    // Check the response status code and handle it as needed
    if response.status().is_success() {
        // Successful response
        println!("Request successful!");
    } else {
        // Handle other response cases
        println!("Request failed with status: {:?}", response.status());
    }

    Ok(())
}
