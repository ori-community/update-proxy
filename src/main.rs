use axum::routing::{get, post};
use axum::{Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use crate::application_state::ApplicationState;
use crate::shutdown_signal::shutdown_signal;

mod http;
mod shutdown_signal;
mod application_state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let application_state = ApplicationState::new();

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/releases", get(http::wotw_releases::handler))  // Legacy v4 route
        .route("/releases/wotw", get(http::wotw_releases::handler))
        .route("/motd/wotw", get(http::wotw_motd::handler))
        .route("/wotw-community-patch/latest", get(http::wotw_community_patch_releases::handler))
        .route("/clear-cache", post(http::clear_cache::handler))
        .with_state(application_state.clone())
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server is listening on 0.0.0.0:3000");
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.unwrap();
}
