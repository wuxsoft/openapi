use std::sync::Arc;

use tokio::runtime::Runtime;

use crate::{Config, Result, content::{ContentContext, NewsItem, TopicItem}};

/// Blocking content context
pub struct ContentContextSync {
    ctx: ContentContext,
    rt: Runtime,
}

impl ContentContextSync {
    /// Create a `ContentContextSync`
    pub fn new(config: Arc<Config>) -> Self {
        let ctx = ContentContext::new(config);
        let rt = Runtime::new().expect("create tokio runtime");
        Self { ctx, rt }
    }

    /// Get discussion topics list
    pub fn topics(&self, symbol: impl Into<String>) -> Result<Vec<TopicItem>> {
        self.rt.block_on(self.ctx.topics(symbol))
    }

    /// Get news list
    pub fn news(&self, symbol: impl Into<String>) -> Result<Vec<NewsItem>> {
        self.rt.block_on(self.ctx.news(symbol))
    }
}
