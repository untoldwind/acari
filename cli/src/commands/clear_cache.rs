use acari_lib::{AcariError, CachedClient};

pub fn clear_cache() -> Result<(), AcariError> {
  CachedClient::clear_cache()
}
