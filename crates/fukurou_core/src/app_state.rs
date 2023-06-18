use crate::{cache::Cache, config::Config};
use std::{collections::HashMap, sync::Arc};

///
pub struct AppState {
    config: Config,
    cache: parking_lot::Mutex<HashMap<&'static str, Arc<Cache>>>,
}

///
impl AppState {
    pub fn config(&self) -> &Config {
        &self.config
    }

    ///
    pub fn cache<T>(&self) -> Arc<Cache>
    where
        T: ?Sized,
    {
        let typename = std::any::type_name::<T>();
        let mut cache_path = self.config.cache_path();
        cache_path.push(typename);

        self.cache
            .lock()
            .entry(typename)
            .or_insert_with(|| Arc::new(Cache::new(&cache_path)))
            .clone()
    }
}
