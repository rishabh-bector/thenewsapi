# News API Rust

A Rust client library for interacting with the News API, providing access to global news from thousands of sources with exceptional response times.

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

## 
Usage
First, create a client instance with your API token:

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

### Fetch Latest Headlines

```rust
use thenewsapi::{Client, HeadlinesParams};

#[tokio::main]
async fn main() {
    let api_token = "your_api_token";
    let client = Client::new(api_token);

    let params = HeadlinesParams::default();
    let headlines = client.get_headlines(params).await.unwrap();
    println!("{:?}", headlines);
}
```

### Retrieve Top Stories

```rust
use thenewsapi::{Client, TopStoriesParams};

#[tokio::main]
async fn main() {
    let api_token = "your_api_token";
    let client = Client::new(api_token);

    let params = TopStoriesParams::default();
    let top_stories = client.get_top_stories(params).await.unwrap();
    println!("{:?}", top_stories);
}
```

### Access All News Articles

```rust
use thenewsapi::{Client, AllNewsParams};

#[tokio::main]
async fn main() {
    let api_token = "your_api_token";
    let client = Client::new(api_token);

    let params = AllNewsParams::default();
    let all_news = client.get_all_news(params).await.unwrap();
    println!("{:?}", all_news);
}
```

### Find Similar News Articles

```rust
use thenewsapi::{Client, SimilarNewsParams};

#[tokio::main]
async fn main() {
    let api_token = "your_api_token";
    let client = Client::new(api_token);

    let params = SimilarNewsParams::default();
    let similar_news = client.get_similar_news("article_uuid", params).await.unwrap();
    println!("{:?}", similar_news);
}
```

### Retrieve Specific Article by UUID

```rust
use thenewsapi::Client;

#[tokio::main]
async fn main() {
    let api_token = "your_api_token";
    let client = Client::new(api_token);

    let article = client.get_article_by_uuid("article_uuid").await.unwrap();
    println!("{:?}", article);
}
```

### List Available News Sources

```rust
use thenewsapi::{Client, SourcesParams};

#[tokio::main]
async fn main() {
    let api_token = "your_api_token";
    let client = Client::new(api_token);

    let params = SourcesParams::default();
    let sources = client.get_sources(params).await.unwrap();
    println!("{:?}", sources);
}
```

## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Contribution
Contributions are welcome! Please open an issue or submit a pull request on GitHub.