use crate::application_state::{ApplicationState, CacheEntry};
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use cached::Cached;

pub async fn handler(State(state): State<ApplicationState>) -> Response {
    let cached_response = state
        .responses_cache
        .lock()
        .await
        .cache_get(&CacheEntry::WotwMotd)
        .cloned();

    let response_body = if let Some(cached_response_body) = cached_response {
        cached_response_body
    } else {
        let octocrab = octocrab::instance();

        let response_body = serde_json::to_string(
            &match octocrab
                .repos("ori-community", "motd")
                .get_content()
                .path("motd.wotw.html")
                .send()
                .await
            {
                Ok(x) => x,
                Err(err) => {
                    log::error!("Error fetching WotW MotD from GitHub: {}", err);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response()
                },
            }
            .items,
        )
        .unwrap();

        state
            .responses_cache
            .lock()
            .await
            .cache_set(CacheEntry::WotwMotd, response_body.clone());

        response_body
    };

    let mut response = Response::new(Body::new(response_body));
    response.headers_mut().insert("Content-Type", HeaderValue::from_static("application/json"));
    response
}
