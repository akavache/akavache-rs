use futures;
use time;

use time::Timespec;

struct CacheEntry {
  created_at: Timespec,
  expires_at: Option<Timespec>,
  value: Vec<u8>,
}

impl CacheEntry {
  fn new(val: &[u8], created_at: Option<Timespec>, expires_at: Option<Timespec>) -> CacheEntry {
    CacheEntry { 
      value: val.to_vec(),
      created_at: created_at.unwrap_or(time::now().to_timespec()),
      expires_at: expires_at
    }
  }
}

pub struct InMemoryBlobCache {
}

impl InMemoryBlobCache {
   fn new() -> InMemoryBlobCache {
     return InMemoryBlobCache {}
   }
}

impl BlobCache for InMemoryBlobCache {
  fn get(key: &str) -> ErrFuture<Vec<u8>> {

  }

  fn get_all_keys() -> ErrFuture<Vec<String>> {

  }

  fn get_created_at(key: &str, absoluteExpiration: Option<Timespec>) -> ErrFuture<Timespec> {

  }

  fn insert(key : &str, value: &[u8]) -> ErrFuture<bool> {

  }

  fn invalidate(key: &str) -> ErrFuture<bool> {

  }

  fn invalidate_all() -> ErrFuture<bool> {

  }

  fn flush() -> ErrFuture<bool>;
  fn vacuum() -> ErrFuture<bool>;
  fn shutdown() -> ErrFuture<bool>;
}
*/