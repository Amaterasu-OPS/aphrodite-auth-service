use crate::application::spi::cache::{CacheFactory, CacheInterface};

pub struct CacheAdapter;

impl CacheAdapter {
    pub fn get_cache_connection<T>() -> Result<T, String>
    where
        T: CacheInterface + CacheFactory,
    {
        T::get()
    }
}