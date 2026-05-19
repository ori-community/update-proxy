use crate::application_state::{ApplicationState, CacheEntry};
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
struct MotdResponse {
    motd: String,
}

pub async fn handler(State(state): State<ApplicationState>) -> Response {
    let response_body = match state
        .get_or_set_cache_with(
            CacheEntry::WotwReleases,
            async || -> Result<String, octocrab::Error> {
                let octocrab = octocrab::instance();

                let response_body = serde_json::to_string(&MotdResponse {
                    motd: octocrab
                        .repos("ori-community", "motd")
                        .get_content()
                        .path("motd.wotw.html")
                        .send()
                        .await?
                        .items
                        .into_iter()
                        .next()
                        .unwrap()
                        .decoded_content()
                        .unwrap(),
                })
                .unwrap();

                Ok(response_body)
            },
        )
        .await
    {
        Ok(body) => body,
        Err(err) => {
            log::error!("Error fetching WotW MotD from GitHub: {}", err);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut response = Response::new(Body::new(response_body));
    response
        .headers_mut()
        .insert("Content-Type", HeaderValue::from_static("application/json"));
    response
}
