# The News API

A Rust client library for interacting with The News API (https://www.thenewsapi.com/), providing access to global news from thousands of sources with exceptional response times.

## Features

- Fetch latest headlines by category
- Retrieve top stories
- Access all news articles with advanced filtering
- Find similar news articles based on a specific article UUID
- Retrieve specific articles by UUID
- List available news sources

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
thenewsapi = "0.1.0"
```

Ensure you also have the required dependencies:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
```

## Usage

```rust
use thenewsapi::Client;

#[tokio::main]
async fn main() {
    let api_token = "your_api_token";
    let client = Client::new(api_token);

    // Example: Fetch latest headlines
    let params = HeadlinesParams::default();
    match client.get_headlines(params).await {
        Ok(headlines) => println!("{:?}", headlines),
        Err(e) => eprintln!("Error fetching headlines: {:?}", e),
    }
}
```

See the docs for all supported methods and parameters.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Contribution
Contributions are welcome. Please open an issue or submit a pull request on GitHub.