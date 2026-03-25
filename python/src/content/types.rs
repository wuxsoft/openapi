use longbridge_python_macros::PyObject;
use pyo3::prelude::*;

use crate::time::PyOffsetDateTimeWrapper;

/// Topic author
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::content::TopicAuthor")]
pub(crate) struct TopicAuthor {
    /// Member ID
    member_id: String,
    /// Display name
    name: String,
    /// Avatar URL
    avatar: String,
}

/// Topic image
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::content::TopicImage")]
pub(crate) struct TopicImage {
    /// Original image URL
    url: String,
    /// Small thumbnail URL
    sm: String,
    /// Large image URL
    lg: String,
}

/// My topic item (topic created by the current authenticated user)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::content::OwnedTopic")]
pub(crate) struct OwnedTopic {
    /// Topic ID
    id: String,
    /// Title
    title: String,
    /// Plain text excerpt
    description: String,
    /// Markdown body
    body: String,
    /// Author
    author: TopicAuthor,
    /// Related stock tickers
    #[py(array)]
    tickers: Vec<String>,
    /// Hashtag names
    #[py(array)]
    hashtags: Vec<String>,
    /// Images
    #[py(array)]
    images: Vec<TopicImage>,
    /// Likes count
    likes_count: i32,
    /// Comments count
    comments_count: i32,
    /// Views count
    views_count: i32,
    /// Shares count
    shares_count: i32,
    /// Content type: "article" or "post"
    topic_type: String,
    /// License: 0=none, 1=original, 2=non-original
    license: i32,
    /// URL to the full topic page
    detail_url: String,
    /// Created time
    created_at: PyOffsetDateTimeWrapper,
    /// Updated time
    updated_at: PyOffsetDateTimeWrapper,
}

/// Topic item
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::content::TopicItem")]
pub(crate) struct TopicItem {
    /// Topic ID
    id: String,
    /// Title
    title: String,
    /// Description
    description: String,
    /// URL
    url: String,
    /// Published time
    published_at: PyOffsetDateTimeWrapper,
    /// Comments count
    comments_count: i32,
    /// Likes count
    likes_count: i32,
    /// Shares count
    shares_count: i32,
}

/// News item
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::content::NewsItem")]
pub(crate) struct NewsItem {
    /// News ID
    id: String,
    /// Title
    title: String,
    /// Description
    description: String,
    /// URL
    url: String,
    /// Published time
    published_at: PyOffsetDateTimeWrapper,
    /// Comments count
    comments_count: i32,
    /// Likes count
    likes_count: i32,
    /// Shares count
    shares_count: i32,
}
