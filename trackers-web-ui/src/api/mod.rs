use std::sync::LazyLock;

use leptos::*;

pub static API_SERVER_WEB_HOST: LazyLock<&'static str> = std::sync::LazyLock::new(|| {
    #[cfg(feature = "local-dev")]
    let host = dotenvy::var("API_SERVER_WEB_HOST")
        .expect("API_SERVER_WEB_HOST environment variable must be set in the .env file")
        .leak();
    #[cfg(not(feature = "local-dev"))]
    let host = std::env::var("API_SERVER_WEB_HOST")
        .expect("Expected API_SERVER_WEB_HOST to be set in the environmnet")
        .leak();

    host
});

pub static API_SERVER_LOCAL_HOST: LazyLock<&'static str> = std::sync::LazyLock::new(|| {
    #[cfg(feature = "local-dev")]
    let host = dotenvy::var("API_SERVER_LOCAL_HOST")
        .expect("API_SERVER_LOCAL_HOST environment variable must be set in the .env file")
        .leak();
    #[cfg(not(feature = "local-dev"))]
    let host = std::env::var("API_SERVER_LOCAL_HOST")
        .expect("Expected API_SERVER_LOCAL_HOST to be set in the environmnet")
        .leak();

    host
});

#[cfg(not(feature = "ssr"))]
pub async fn fetch_api<T>(cx: Scope, path: &str) -> Option<T>
where
    T: Serializable,
{
    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    let api_host = { String::from("http://") + &API_SERVER_WEB_HOST };
    log!("{api_host}");

    let json = gloo_net::http::Request::get(&(api_host + path))
        .abort_signal(abort_signal.as_ref())
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;

    // abort in-flight requests if the Scope is disposed
    // i.e., if we've navigated away from this page
    on_cleanup(cx, move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });
    T::de(&json).ok()
}

/// Path should not contain the host. Host is read from environment variables.
#[cfg(feature = "ssr")]
pub async fn fetch_api<T>(cx: Scope, path: &str) -> Option<T>
where
    T: Serializable,
{
    let api_host = { String::from("http://") + &API_SERVER_LOCAL_HOST };
    log!("{api_host}");

    let json = reqwest::get(&(api_host + path))
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;
    T::de(&json).map_err(|e| log::error!("{e}")).ok()
}
