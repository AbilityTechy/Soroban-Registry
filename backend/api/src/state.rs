use crate::auth::AuthManager;
use crate::cache::{CacheConfig, CacheLayer};
use crate::resource_tracking::ResourceManager;
use prometheus::Registry;
use sqlx::PgPool;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Instant;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub started_at: Instant,
    pub cache: Arc<CacheLayer>,
    pub registry: Registry,
    pub resource_mgr: Arc<RwLock<ResourceManager>>,
    pub auth_mgr: Arc<RwLock<AuthManager>>,
}

impl AppState {
    pub fn new(db: PgPool, registry: Registry) -> Self {
        let config = CacheConfig::from_env();
        Self {
            db,
            started_at: Instant::now(),
            cache: Arc::new(CacheLayer::new(config)),
            registry,
            resource_mgr: Arc::new(RwLock::new(ResourceManager::new())),
            auth_mgr: Arc::new(RwLock::new(AuthManager::from_env())),
        }
    }
}
