use std::{cell::Cell, time::Duration};

// because we may call `is_cn` multi times in a short time, we cache the result
thread_local! {
    static IS_CN: Cell<Option<bool>> = const { Cell::new(None) };
}

/// do the best to guess whether the access point is in China Mainland or not
pub async fn is_cn() -> bool {
    // check user defined REGION (LONGBRIDGE_REGION takes precedence,
    // LONGPORT_REGION is the fallback)
    let user_region = std::env::var("LONGBRIDGE_REGION")
        .ok()
        .or_else(|| std::env::var("LONGPORT_REGION").ok());
    if let Some(region) = user_region {
        return region.eq_ignore_ascii_case("CN");
    }

    // return cached result if available
    if let Some(cached) = IS_CN.get() {
        return cached;
    }

    // probe: HTTP 200 means CN, anything else (error or non-200) means not CN
    let result = reqwest::Client::new()
        .get("https://geotest.lbkrs.com")
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .is_ok_and(|resp| resp.status().is_success());

    IS_CN.set(Some(result));
    result
}
