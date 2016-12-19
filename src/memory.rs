use futures::future::*;
use time;

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::iter::FromIterator;
use std::sync::Mutex;
use time::Timespec;
use traits::{BlobCache, ErrFuture};

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
  data: HashMap<String, CacheEntry>,
  lock: Mutex<bool>,
}

impl InMemoryBlobCache {
   pub fn new() -> InMemoryBlobCache {
     return InMemoryBlobCache { data: HashMap::new(), lock: Mutex::new(false) };
   }
}

impl<'a> BlobCache<'a> for InMemoryBlobCache {
  fn get(&mut self, key: &str) -> ErrFuture<Vec<u8>> {
    let _l = self.lock.lock().unwrap();

    if let Some(ce) = self.data.get(key) {
      match ce.expires_at {
        Some(e) if e < time::now().to_timespec() => 
          return failed(Error::new(ErrorKind::Other, "Key not found")).boxed(),
        _ => return done(Ok(ce.value.clone())).boxed(),
      }
    } else {
      return failed(Error::new(ErrorKind::Other, "Key not found")).boxed();
    }
  }

  fn get_all_keys(&mut self) -> ErrFuture<Vec<String>> {
    let _l = self.lock.lock().unwrap();
    let ret = Vec::from_iter(self.data.keys().map(|x| x.to_owned()));
    return done(Ok(ret)).boxed();
  }

  fn get_created_at(&mut self, key: &str) -> ErrFuture<Timespec> {
    let _l = self.lock.lock().unwrap();

    if let Some(x) = self.data.get(key) {
      return done(Ok(x.created_at)).boxed();
    } else {
      return failed(Error::new(ErrorKind::Other, "Key not found")).boxed();
    }
  }

  fn insert(&mut self, key : &str, value: &[u8], expires_at: Option<Timespec>) -> ErrFuture<bool> {
    let _l = self.lock.lock().unwrap();

    self.data.insert(key.to_owned(), CacheEntry::new(value, None, expires_at));
    return done(Ok(true)).boxed();
  }

  fn invalidate(&mut self, key: &str) -> ErrFuture<bool> {
    let _l = self.lock.lock().unwrap();

    if let Some(_) = self.data.remove(key) {
      return done(Ok(true)).boxed();
    } else {
      return failed(Error::new(ErrorKind::Other, "Key not found")).boxed();
    }
  }

  fn invalidate_all(&mut self) -> ErrFuture<bool> {
    let _l = self.lock.lock().unwrap();

    self.data.clear();
    return done(Ok(true)).boxed();
  }

  fn flush(&mut self) -> ErrFuture<bool> {
    let _l = self.lock.lock().unwrap();
    return done(Ok(true)).boxed();
  }

  fn vacuum(&mut self) -> ErrFuture<bool> {
    let _l = self.lock.lock().unwrap();
    return done(Ok(true)).boxed();
  }

  fn shutdown(&mut self) -> ErrFuture<bool> {
    let _l = self.lock.lock().unwrap();
    return done(Ok(true)).boxed();
  }
}