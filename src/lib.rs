//! # News API Client
//!
//! This crate provides a client library for interacting with the News API.
//!
//! The News API provides global news from thousands of sources with exceptional response times. This client allows you to access headlines, top stories, all news articles, similar news articles, and sources.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! news_api_client = "0.1.0"
//! reqwest = { version = "0.11", features = ["json"] }
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! tokio = { version = "1.0", features = ["full"] }
//! anyhow = "1.0"
//! ```
//!
//! ## Examples
//!
//! ```rust,no_run
//! use news_api_client::{Client, HeadlinesParams};
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_token = "your_api_token";
//!     let client = Client::new(api_token);
//!
//!     let params = HeadlinesParams::default();
//!     let headlines = client.get_headlines(params).await.unwrap();
//!     println!("{:?}", headlines);
//! }
//! ```

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

/// Represents an article in the News API.
#[derive(Deserialize, Debug)]
pub struct Article {
    /// The unique identifier for an article.
    pub uuid: String,
    /// The article title.
    pub title: String,
    /// The article meta description.
    pub description: String,
    /// The article meta keywords.
    pub keywords: Option<String>,
    /// The first 60 characters of the article body.
    pub snippet: String,
    /// The URL to the article.
    pub url: String,
    /// The URL to the article image.
    pub image_url: Option<String>,
    /// The language of the source.
    pub language: String,
    /// The datetime the article was published.
    pub published_at: String,
    /// The domain of the source.
    pub source: String,
    /// Array of strings which the source is categorized as.
    pub categories: Vec<String>,
    /// Locale of the source.
    pub locale: Option<String>,
    /// An array of similar articles to the base article.
    pub similar: Option<Vec<SimilarArticle>>,
}

/// Represents a similar article in the News API.
#[derive(Deserialize, Debug)]
pub struct SimilarArticle {
    /// The unique identifier for a similar article.
    pub uuid: String,
    /// The article title.
    pub title: String,
    /// The article meta description.
    pub description: String,
    /// The article meta keywords.
    pub keywords: Option<String>,
    /// The first 60 characters of the article body.
    pub snippet: String,
    /// The URL to the article.
    pub url: String,
    /// The URL to the article image.
    pub image_url: Option<String>,
    /// The language of the source.
    pub language: String,
    /// The datetime the article was published.
    pub published_at: String,
    /// The domain of the source.
    pub source: String,
    /// Array of strings which the source is categorized as.
    pub categories: Vec<String>,
    /// Locale of the source.
    pub locale: Option<String>,
}

/// Represents metadata in the News API responses.
#[derive(Deserialize, Debug)]
pub struct Meta {
    /// The number of articles found for the request.
    pub found: usize,
    /// The number of articles returned on the page.
    pub returned: usize,
    /// The limit based on the limit parameter.
    pub limit: usize,
    /// The page number based on the page parameter.
    pub page: usize,
}

/// Represents the response for headlines in the News API.
#[derive(Deserialize, Debug)]
pub struct HeadlinesResponse {
    /// The data containing the headlines.
    pub data: HashMap<String, Vec<Article>>,
}

/// Represents the response for top stories in the News API.
#[derive(Deserialize, Debug)]
pub struct TopStoriesResponse {
    /// Metadata about the response.
    pub meta: Meta,
    /// The data containing the top stories.
    pub data: Vec<Article>,
}

/// Represents the response for all news articles in the News API.
#[derive(Deserialize, Debug)]
pub struct AllNewsResponse {
    /// Metadata about the response.
    pub meta: Meta,
    /// The data containing all news articles.
    pub data: Vec<Article>,
}

/// Represents the response for similar news articles in the News API.
#[derive(Deserialize, Debug)]
pub struct SimilarNewsResponse {
    /// Metadata about the response.
    pub meta: Meta,
    /// The data containing similar news articles.
    pub data: Vec<Article>,
}

/// Represents the response for a specific article by UUID in the News API.
#[derive(Deserialize, Debug)]
pub struct ArticleByUuidResponse {
    /// The unique identifier for an article.
    pub uuid: String,
    /// The article title.
    pub title: String,
    /// The article meta description.
    pub description: String,
    /// The article meta keywords.
    pub keywords: Option<String>,
    /// The first 60 characters of the article body.
    pub snippet: String,
    /// The URL to the article.
    pub url: String,
    /// The URL to the article image.
    pub image_url: Option<String>,
    /// The language of the source.
    pub language: String,
    /// The datetime the article was published.
    pub published_at: String,
    /// The domain of the source.
    pub source: String,
    /// Array of strings which the source is categorized as.
    pub categories: Vec<String>,
}

/// Represents a source in the News API.
#[derive(Deserialize, Debug)]
pub struct Source {
    /// The unique ID of the source feed.
    pub source_id: String,
    /// The domain of the source.
    pub domain: String,
    /// The source language.
    pub language: String,
    /// The source locale.
    pub locale: Option<String>,
    /// Array of strings which the source is categorized as.
    pub categories: Vec<String>,
}

/// Represents the response for sources in the News API.
#[derive(Deserialize, Debug)]
pub struct SourcesResponse {
    /// Metadata about the response.
    pub meta: Meta,
    /// The data containing the sources.
    pub data: Vec<Source>,
}

/// Parameters for the `get_headlines` API call.
#[derive(Serialize, Default)]
pub struct HeadlinesParams<'a> {
    pub locale: Option<&'a str>,
    pub domains: Option<&'a str>,
    pub exclude_domains: Option<&'a str>,
    pub source_ids: Option<&'a str>,
    pub exclude_source_ids: Option<&'a str>,
    pub language: Option<&'a str>,
    pub published_on: Option<&'a str>,
    pub headlines_per_category: Option<usize>,
    pub include_similar: Option<bool>,
}

/// Parameters for the `get_top_stories` API call.
#[derive(Serialize, Default)]
pub struct TopStoriesParams<'a> {
    pub search: Option<&'a str>,
    pub search_fields: Option<&'a str>,
    pub locale: Option<&'a str>,
    pub categories: Option<&'a str>,
    pub exclude_categories: Option<&'a str>,
    pub domains: Option<&'a str>,
    pub exclude_domains: Option<&'a str>,
    pub source_ids: Option<&'a str>,
    pub exclude_source_ids: Option<&'a str>,
    pub language: Option<&'a str>,
    pub published_before: Option<&'a str>,
    pub published_after: Option<&'a str>,
    pub published_on: Option<&'a str>,
    pub sort: Option<&'a str>,
    pub limit: Option<usize>,
    pub page: Option<usize>,
}

/// Parameters for the `get_all_news` API call.
#[derive(Serialize, Default)]
pub struct AllNewsParams<'a> {
    pub search: Option<&'a str>,
    pub search_fields: Option<&'a str>,
    pub locale: Option<&'a str>,
    pub categories: Option<&'a str>,
    pub exclude_categories: Option<&'a str>,
    pub domains: Option<&'a str>,
    pub exclude_domains: Option<&'a str>,
    pub source_ids: Option<&'a str>,
    pub exclude_source_ids: Option<&'a str>,
    pub language: Option<&'a str>,
    pub published_before: Option<&'a str>,
    pub published_after: Option<&'a str>,
    pub published_on: Option<&'a str>,
    pub sort: Option<&'a str>,
    pub limit: Option<usize>,
    pub page: Option<usize>,
}

/// Parameters for the `get_similar_news` API call.
#[derive(Serialize, Default)]
pub struct SimilarNewsParams<'a> {
    pub categories: Option<&'a str>,
    pub exclude_categories: Option<&'a str>,
    pub domains: Option<&'a str>,
    pub exclude_domains: Option<&'a str>,
    pub source_ids: Option<&'a str>,
    pub exclude_source_ids: Option<&'a str>,
    pub language: Option<&'a str>,
    pub published_before: Option<&'a str>,
    pub published_after: Option<&'a str>,
    pub published_on: Option<&'a str>,
    pub limit: Option<usize>,
    pub page: Option<usize>,
}

/// Parameters for the `get_sources` API call.
#[derive(Serialize, Default)]
pub struct SourcesParams<'a> {
    pub categories: Option<&'a str>,
    pub exclude_categories: Option<&'a str>,
    pub language: Option<&'a str>,
    pub page: Option<usize>,
}

/// A client for the News API.
pub struct Client {
    client: reqwest::Client,
    api_token: String,
}

impl Client {
    /// Creates a new News API client.
    ///
    /// # Arguments
    ///
    /// * `api_token` - Your API token which can be found on your account dashboard.
    ///
    /// # Example
    ///
    /// ```rust
    /// let client = Client::new("your_api_token");
    /// ```
    pub fn new(api_token: &str) -> Self {
        Client {
            client: reqwest::Client::new(),
            api_token: api_token.to_string(),
        }
    }

    /// Gets the latest headlines.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters to filter the headlines.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let params = HeadlinesParams::default();
    /// let headlines = client.get_headlines(params).await.unwrap();
    /// println!("{:?}", headlines);
    /// ```
    pub async fn get_headlines(&self, params: HeadlinesParams<'_>) -> Result<HeadlinesResponse> {
        self.get("https://api.thenewsapi.com/v1/news/headlines", params)
            .await
    }

    /// Gets the top stories.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters to filter the top stories.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let params = TopStoriesParams::default();
    /// let top_stories = client.get_top_stories(params).await.unwrap();
    /// println!("{:?}", top_stories);
    /// ```
    pub async fn get_top_stories(
        &self,
        params: TopStoriesParams<'_>,
    ) -> Result<TopStoriesResponse> {
        self.get("https://api.thenewsapi.com/v1/news/top", params)
            .await
    }

    /// Gets all news articles.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters to filter all news articles.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let params = AllNewsParams::default();
    /// let all_news = client.get_all_news(params).await.unwrap();
    /// println!("{:?}", all_news);
    /// ```
    pub async fn get_all_news(&self, params: AllNewsParams<'_>) -> Result<AllNewsResponse> {
        self.get("https://api.thenewsapi.com/v1/news/all", params)
            .await
    }

    /// Gets similar news articles based on a specific article UUID.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The UUID of the article to find similar articles for.
    /// * `params` - Parameters to filter the similar news articles.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let params = SimilarNewsParams::default();
    /// ```
    pub async fn get_similar_news(
        &self,
        uuid: &str,
        params: SimilarNewsParams<'_>,
    ) -> Result<SimilarNewsResponse> {
        self.get(
            &format!("https://api.thenewsapi.com/v1/news/similar/{}", uuid),
            params,
        )
        .await
    }

    /// Gets a specific article by UUID.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The UUID of the article to retrieve.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let article = client.get_article_by_uuid("article_uuid").await.unwrap();
    /// println!("{:?}", article);
    /// ```
    pub async fn get_article_by_uuid(&self, uuid: &str) -> Result<ArticleByUuidResponse> {
        self.get(
            &format!("https://api.thenewsapi.com/v1/news/uuid/{}", uuid),
            (),
        )
        .await
    }

    /// Gets the sources.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters to filter the sources.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let params = SourcesParams::default();
    /// let sources = client.get_sources(params).await.unwrap();
    /// println!("{:?}", sources);
    /// ```
    pub async fn get_sources(&self, params: SourcesParams<'_>) -> Result<SourcesResponse> {
        self.get("https://api.thenewsapi.com/v1/sources", params)
            .await
    }

    async fn get<T: Serialize, U: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        params: T,
    ) -> Result<U> {
        let query = self.build_query(params);
        let response = self.client.get(url).query(&query).send().await?;

        if response.status().is_success() {
            let parsed_response = response.json::<U>().await?;
            Ok(parsed_response)
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| String::from("Failed to read response text"));
            Err(anyhow!("HTTP {}: {}", status, error_text))
        }
    }

    fn build_query<T: Serialize>(&self, params: T) -> HashMap<String, String> {
        let mut query = serde_json::to_value(params)
            .unwrap_or(json!({}))
            .as_object()
            .unwrap()
            .clone();

        query.insert("api_token".to_string(), json!(self.api_token));

        query
            .into_iter()
            .filter(|(_, v)| !v.is_null())
            .map(|(k, v)| {
                let value_str =
                    if k == "published_on" || k == "published_before" || k == "published_after" {
                        format!("{}", v.as_str().unwrap_or("").to_string())
                    } else {
                        v.as_str().unwrap_or("").to_string()
                    };
                (k, value_str)
            })
            .collect()
    }
}
