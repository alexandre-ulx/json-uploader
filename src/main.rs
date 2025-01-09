use serde_json::Value;
use std::path::Path;
use reqwest::Client;
use tokio::fs as async_fs;
use std::env;
use url::Url;

async fn send_file(client: &Client, endpoint: &str, token: &str, file_path: &Path, success_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Read the file content
    let file_content = tokio::fs::read_to_string(file_path).await?;

    // Parse JSON to validate structure
    let json_data: Value = serde_json::from_str(&file_content)?;

    // Send the data to the endpoint
    let response = client
        .post(endpoint)
        .bearer_auth(token)
        .json(&json_data)
        .send()
        .await?;

    if response.status().is_success() {
        // Move the file to the success directory
        let file_name = file_path.file_name().unwrap();
        let new_path = success_dir.join(file_name);
        async_fs::rename(file_path, new_path).await?;
        println!("Successfully sent and moved file: {:?}", file_name);
    } else {
        println!("Failed to send file: {:?}, Status: {}", file_path, response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        eprintln!("Usage: {} <TOKEN> <ENDPOINT> <INPUT_DIR> <SUCCESS_DIR> <ERROR_DIR>", args[0]);
        std::process::exit(1);
    }

    let token = &args[1];
    let endpoint = &args[2];
    let input_dir = Path::new(&args[3]);
    let success_dir = Path::new(&args[4]);
    let error_dir = Path::new(&args[5]);

    // Validate the endpoint as a URL
    if Url::parse(endpoint).is_err() {
        eprintln!("Invalid endpoint URL: {}", endpoint);
        std::process::exit(1);
    }

    // Create the success directory if it does not exist
    if !success_dir.exists() {
        tokio::fs::create_dir_all(success_dir).await?;
    }

    // Create an HTTP client
    let client = Client::new();

    // Iterate over all files in the input directory
    for entry in std::fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            if let Err(e) = send_file(&client, endpoint, token, &path, success_dir).await {
                eprintln!("Error processing file {:?}: {}", path, e);
                // Move the file to the error directory
                let file_name = path.file_name().unwrap();
                let error_dir_str = format!("{}", error_dir.display());
                let error_dir = Path::new(&error_dir_str);
                if !error_dir.exists() {
                    tokio::fs::create_dir_all(error_dir).await?;
                }
                let filename = path.to_str().unwrap();
                let new_path = error_dir.join(file_name);
                async_fs::rename(filename, new_path).await?;
            }
        }
    }

    Ok(())
}