use crate::application_state::{ApplicationState, CacheEntry};
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use cached::Cached;

pub async fn handler(State(state): State<ApplicationState>) -> Response {
    let cached_response = state
        .responses_cache
        .lock()
        .await
        .cache_get(&CacheEntry::WotwMotd)
        .cloned();

    let response = if let Some(cached_response) = cached_response {
        cached_response
    } else {
        let octocrab = octocrab::instance();

        let response = serde_json::to_string(
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
            .cache_set(CacheEntry::WotwMotd, response.clone());

        response
    };

    Response::new(Body::new(response))
}
