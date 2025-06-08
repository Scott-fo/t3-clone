use axum::routing::{get, post};
use axum::{Router, middleware};
use secrecy::ExposeSecret;
use tower::ServiceBuilder;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tower_sessions::{
    SessionManagerLayer,
    cookie::{Key, SameSite, time::Duration},
};
use tower_sessions_redis_store::RedisStore;

use crate::handlers::auth::get_current_user;
use crate::{
    app::AppState,
    handlers::auth::{login, logout, register},
};

pub fn app_routes(app_state: AppState) -> Router {
    let session_store = RedisStore::new(app_state.cache.clone());
    let session_service = SessionManagerLayer::new(session_store)
        .with_signed(Key::from(
            app_state
                .config
                .application
                .secret
                .expose_secret()
                .as_bytes(),
        ))
        .with_name("session")
        .with_secure(true)
        .with_http_only(true)
        .with_same_site(SameSite::Strict)
        .with_expiry(tower_sessions::Expiry::OnInactivity(Duration::seconds(
            30 * 24 * 60 * 60,
        )));

    Router::new()
        .nest("/api", protected_routes(app_state.clone()))
        .nest("/auth", auth_routes())
        .fallback_service(
            ServeDir::new("./frontend/dist")
                .not_found_service(ServeFile::new("./frontend/dist/index.html")),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(session_service),
        )
        .with_state(app_state)
}

pub fn auth_routes() -> Router<AppState> {
    let governor_config = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    let governor = GovernorLayer {
        config: governor_config.into(),
    };

    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .layer(governor)
}

pub fn protected_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(get_current_user))
        .route("/logout", post(logout))
        .route_layer(middleware::from_fn_with_state(
            state,
            crate::middleware::auth::auth,
        ))
}
