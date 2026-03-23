use std::sync::Arc;

use longbridge::content::ContentContext;
use pyo3::{prelude::*, types::PyType};

use crate::{
    config::Config,
    content::types::{NewsItem, TopicItem},
    error::ErrorNewType,
};

/// Async content context.
#[pyclass]
pub(crate) struct AsyncContentContext {
    ctx: Arc<ContentContext>,
}

#[pymethods]
impl AsyncContentContext {
    /// Create an async content context.
    #[classmethod]
    fn create(_cls: &Bound<PyType>, config: &Config) -> Self {
        AsyncContentContext {
            ctx: Arc::new(ContentContext::new(Arc::new(config.0.clone()))),
        }
    }

    /// Get discussion topics list. Returns awaitable.
    fn topics(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.topics(symbol).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<TopicItem> { x.try_into() })
                .collect::<PyResult<Vec<TopicItem>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get news list. Returns awaitable.
    fn news(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.news(symbol).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<NewsItem> { x.try_into() })
                .collect::<PyResult<Vec<NewsItem>>>()
        })
        .map(|b| b.unbind())
    }
}
