use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::content::ContentContext;

use crate::{
    async_call::{CAsyncCallback, execute_async},
    config::CConfig,
    content_context::types::{CNewsItemOwned, CTopicItemOwned},
    types::{CVec, cstr_to_rust},
};

/// Content context
pub struct CContentContext {
    ctx: ContentContext,
}

/// Create a new `ContentContext`
///
/// @param config  Config object
/// @return A new content context
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_content_context_new(config: *const CConfig) -> *const CContentContext {
    let config = Arc::new((*config).0.clone());
    let ctx = ContentContext::new(config);
    Arc::into_raw(Arc::new(CContentContext { ctx }))
}

/// Retain the content context (increment reference count)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_content_context_retain(ctx: *const CContentContext) {
    Arc::increment_strong_count(ctx);
}

/// Release the content context (decrement reference count)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_content_context_release(ctx: *const CContentContext) {
    let _ = Arc::from_raw(ctx);
}

/// Get discussion topics list for a symbol
///
/// @param ctx       Content context
/// @param symbol    Security symbol (e.g. "700.HK")
/// @param callback  Async callback
/// @param userdata  User data passed to the callback
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_content_context_topics(
    ctx: *const CContentContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CTopicItemOwned> = ctx_inner.topics(symbol).await?.into();
        Ok(rows)
    });
}

/// Get news list for a symbol
///
/// @param ctx       Content context
/// @param symbol    Security symbol (e.g. "700.HK")
/// @param callback  Async callback
/// @param userdata  User data passed to the callback
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_content_context_news(
    ctx: *const CContentContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CNewsItemOwned> = ctx_inner.news(symbol).await?.into();
        Ok(rows)
    });
}
