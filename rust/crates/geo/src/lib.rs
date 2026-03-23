//! Geo-detection helper for Longbridge OpenAPI.
//!
//! Determines whether the current access point is in China Mainland so that
//! callers can choose between `*.longbridge.cn` and `*.longbridge.com`
//! endpoints.

use std::{cell::Cell, time::Duration};

// Because `is_cn` may be called many times in quick succession, cache the
// probe result per thread after the first call.
thread_local! {
    static IS_CN: Cell<Option<bool>> = const { Cell::new(None) };
}

/// Do the best to guess whether the access point is in China Mainland or not.
///
/// Detection priority:
/// 1. `LONGBRIDGE_REGION` environment variable (takes precedence).
/// 2. `LONGPORT_REGION` environment variable (fallback alias).
/// 3. Thread-local cached result from a previous probe.
/// 4. Live HTTP probe to `https://geotest.lbkrs.com` — HTTP 200 → CN, anything
///    else (error or non-200) → not CN.
pub async fn is_cn() -> bool {
    // 1 & 2: explicit region override
    let user_region = std::env::var("LONGBRIDGE_REGION")
        .ok()
        .or_else(|| std::env::var("LONGPORT_REGION").ok());
    if let Some(region) = user_region {
        return region.eq_ignore_ascii_case("CN");
    }

    // 3: cached result
    if let Some(cached) = IS_CN.get() {
        return cached;
    }

    // 4: live probe
    let result = reqwest::Client::new()
        .get("https://geotest.lbkrs.com")
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .is_ok_and(|resp| resp.status().is_success());

    IS_CN.set(Some(result));
    result
}
