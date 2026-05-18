use cached::TimedCache;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Eq, PartialEq, Hash)]
pub enum CacheEntry {
    WotwReleases,
    WotwMotd,
}

#[derive(Clone)]
pub struct ApplicationState {
    pub responses_cache: Arc<Mutex<TimedCache<CacheEntry, String>>>,
}

impl ApplicationState {
    pub fn new() -> Self {
        ApplicationState {
            responses_cache: Arc::new(Mutex::new(TimedCache::with_lifespan(Duration::from_hours(
                24,
            )))),
        }
    }
}
