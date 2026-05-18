use std::env::VarError;
use crate::application_state::ApplicationState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use cached::Cached;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClearCacheQuery {
    token: String,
}

pub async fn handler(State(state): State<ApplicationState>, Query(query): Query<ClearCacheQuery>) -> Response {
    match std::env::var("CLEAR_CACHE_TOKEN") {
        Ok(token) => {
            if query.token != token {
                return StatusCode::FORBIDDEN.into_response();
            }
        }
        Err(VarError::NotPresent) => {
            log::error!("CLEAR_CACHE_TOKEN not set, ignoring request");
            return StatusCode::NOT_FOUND.into_response();
        }
        Err(err) => {
            log::error!("Failed to read CLEAR_CACHE_TOKEN: {}", err);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    state
        .responses_cache
        .lock()
        .await
        .cache_clear();

    StatusCode::NO_CONTENT.into_response()
}
