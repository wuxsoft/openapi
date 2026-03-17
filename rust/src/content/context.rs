use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::Deserialize;

use crate::{Config, Result};

use super::types::{NewsItem, TopicItem};

/// Content context
#[derive(Clone)]
pub struct ContentContext(HttpClient);

impl ContentContext {
    /// Create a `ContentContext`
    pub fn new(config: Arc<Config>) -> Self {
        Self(config.create_http_client())
    }

    /// Get discussion topics list
    pub async fn topics(&self, symbol: impl Into<String>) -> Result<Vec<TopicItem>> {
        #[derive(Debug, Deserialize)]
        struct Response {
            items: Vec<TopicItem>,
        }

        let symbol = symbol.into();
        Ok(self
            .0
            .request(Method::GET, format!("/v1/content/{symbol}/topics"))
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .items)
    }

    /// Get news list
    pub async fn news(&self, symbol: impl Into<String>) -> Result<Vec<NewsItem>> {
        #[derive(Debug, Deserialize)]
        struct Response {
            items: Vec<NewsItem>,
        }

        let symbol = symbol.into();
        Ok(self
            .0
            .request(Method::GET, format!("/v1/content/{symbol}/news"))
            .response::<Json<Response>>()
            .send()
            .await?
            .0
            .items)
    }
}
