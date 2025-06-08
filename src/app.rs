use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::configuration::Settings;
use crate::infra;
use crate::routes::app_routes;
use crate::services::container::ServiceContainer;
use crate::services::sse_manager::SseManager;
use tower_sessions_redis_store::fred::prelude::Pool;

#[derive(Debug)]
pub struct ApplicationBaseUrl(pub String);

pub struct Application {
    port: u16,
    listener: tokio::net::TcpListener,
    app: Router,
}

impl Application {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        axum::serve(
            self.listener,
            self.app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
    }

    pub async fn build(config: Settings) -> Result<Self, anyhow::Error> {
        let pool = infra::db::establish_connection(config.application.database_url.clone());
        let cache = infra::redis::establish_connection(config.application.redis_url.clone()).await;

        let addr = format!("{}:{}", config.application.host, config.application.port);
        let addr: SocketAddr = addr.parse()?;
        let port = addr.port();

        let (listener, app) = create(addr, pool, cache, config).await?;

        Ok(Self {
            port,
            listener,
            app,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: infra::db::DbPool,
    pub cache: Pool,
    pub base_url: Arc<ApplicationBaseUrl>,
    pub config: Arc<Settings>,
    pub service_container: Arc<ServiceContainer>,
    pub sse_manager: Arc<SseManager>,
}

async fn create(
    addr: SocketAddr,
    db_pool: infra::db::DbPool,
    cache: Pool,
    config: Settings,
) -> Result<(tokio::net::TcpListener, Router), anyhow::Error> {
    let config = Arc::new(config);
    let base_url = Arc::new(ApplicationBaseUrl(config.application.base_url.clone()));
    let service_container = Arc::new(ServiceContainer::new());
    let sse_manager = Arc::new(SseManager::new());

    let app_state = AppState {
        db_pool,
        cache,
        base_url,
        config,
        service_container,
        sse_manager,
    };

    let app = app_routes(app_state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);

    Ok((listener, app))
}
