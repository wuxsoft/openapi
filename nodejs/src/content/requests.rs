use longbridge::content::{CreateTopicOptions, ListMyTopicsOptions};

/// Options for listing topics created by the current authenticated user
#[napi_derive::napi(object)]
#[derive(Debug, Default)]
pub struct ListMyTopicsRequest {
    /// Page number (default 1)
    pub page: Option<i32>,
    /// Records per page, range 1~500 (default 50)
    pub size: Option<i32>,
    /// Filter by topic type: "article" or "post"; empty returns all
    pub topic_type: Option<String>,
}

impl From<ListMyTopicsRequest> for ListMyTopicsOptions {
    fn from(
        ListMyTopicsRequest {
            page,
            size,
            topic_type,
        }: ListMyTopicsRequest,
    ) -> Self {
        Self {
            page,
            size,
            topic_type,
        }
    }
}

/// Options for creating a topic
#[napi_derive::napi(object)]
#[derive(Debug)]
pub struct CreateTopicRequest {
    /// Topic title (required)
    pub title: String,
    /// Topic body in Markdown format (required)
    pub body: String,
    /// Content type: "article" (long-form) or "post" (short post, default)
    pub topic_type: Option<String>,
    /// Related stock tickers, format: {symbol}.{market}, max 10
    pub tickers: Option<Vec<String>>,
    /// Hashtag names, max 5
    pub hashtags: Option<Vec<String>>,
    /// License: 0=none (default), 1=original, 2=non-original
    pub license: Option<i32>,
}

impl From<CreateTopicRequest> for CreateTopicOptions {
    fn from(
        CreateTopicRequest {
            title,
            body,
            topic_type,
            tickers,
            hashtags,
            license,
        }: CreateTopicRequest,
    ) -> Self {
        Self {
            title,
            body,
            topic_type,
            tickers,
            hashtags,
            license,
        }
    }
}
