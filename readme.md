# JSON File Uploader

This Rust program is designed to upload JSON files from a specified input directory to a provided HTTP endpoint. It validates the JSON files, sends them to the endpoint using a token for authentication, and moves successfully uploaded files to a specified success directory. Files that fail to upload are moved to an error directory.

## Features

- Validates the provided endpoint URL.
- Processes JSON files of any format.
- Authenticates requests using a Bearer token.
- Handles file organization (success and error directories).

## Prerequisites

- [Rust](https://www.rust-lang.org/) (installed and configured)
- Internet access for HTTP requests.

## Usage

### Command-Line Arguments

The program expects the following command-line arguments:

1. `<TOKEN>`: The Bearer token used for authentication.
2. `<ENDPOINT>`: The URL of the HTTP endpoint to send the JSON data.
3. `<INPUT_DIR>`: The directory containing the JSON files to upload.
4. `<SUCCESS_DIR>`: The directory to move successfully uploaded files.

### Example Command

```bash
cargo run -- <TOKEN> <ENDPOINT> <INPUT_DIR> <SUCCESS_DIR>
```

### Example Usage

```bash
cargo run -- eyJhbGciOiJIUzI1Ni... https://api.example.com/upload ./input ./success
```

## Directory Structure

- `INPUT_DIR`: Directory containing the JSON files to be processed.
- `SUCCESS_DIR`: Directory where successfully uploaded files are moved.
- `INPUT_DIR/error`: Subdirectory created automatically for files that fail to upload.

## Code Overview

### File Validation

The program validates if the endpoint is a valid URL using the `url` crate. It also parses the contents of each file to ensure they are valid JSON using the `serde_json` crate.

### File Processing

- JSON files in the `INPUT_DIR` are iterated and validated.
- For each valid file, an HTTP POST request is made to the endpoint with the JSON content.
- On a successful upload, the file is moved to the `SUCCESS_DIR`.
- If the upload fails, the file is moved to the `INPUT_DIR/error` directory.

### Error Handling

- If the endpoint is not a valid URL, the program terminates with an error message.
- If any JSON file fails to upload, it is moved to the `error` directory with an appropriate message printed to the console.

## Dependencies

The program uses the following Rust crates:

- `serde`: For JSON serialization and deserialization.
- `serde_json`: For JSON parsing and validation.
- `reqwest`: For making HTTP requests.
- `tokio`: For asynchronous operations.
- `url`: For validating the endpoint URL.

## How to Build and Run

1. Clone the repository.
2. Navigate to the project directory.
3. Build the project using Cargo:
   ```bash
   cargo build
   ```
4. Run the project using Cargo with the required arguments:
   ```bash
   cargo run -- <TOKEN> <ENDPOINT> <INPUT_DIR> <SUCCESS_DIR>
   ```

## License

This project is licensed under the MIT License.