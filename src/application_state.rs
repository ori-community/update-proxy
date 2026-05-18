use cached::{Cached, TimedCache};
use std::fmt::Display;
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

    /// Returns a cached value for `entry`. If no value exists, runs the future returned by `f`
    /// and stores its result in the cache. If the future returns an error result, nothing is
    /// written to cache and the result is returned as-is.
    pub async fn get_or_set_cache_with<F, FUT, E>(
        &self,
        entry: CacheEntry,
        f: F,
    ) -> Result<String, E>
    where
        F: FnOnce() -> FUT,
        FUT: Future<Output = Result<String, E>>,
        E: Display,
    {
        let cached_response = self.responses_cache.lock().await.cache_get(&entry).cloned();

        if let Some(cached_response_body) = cached_response {
            Ok(cached_response_body)
        } else {
            match f().await {
                Ok(response_body) => {
                    self.responses_cache
                        .lock()
                        .await
                        .cache_set(CacheEntry::WotwMotd, response_body.clone());

                    Ok(response_body)
                }
                Err(error) => Err(error),
            }
        }
    }
}
