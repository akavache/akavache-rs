use std;
use futures;

use time::Timespec;

pub type ErrFuture<T> = Box<futures::Future<Item = T, Error = std::io::Error> + Send>;

pub trait BlobCache<'a> {
  fn get(&self, key: &str) -> ErrFuture<Vec<u8>>;
  fn get_all_keys(&self) -> ErrFuture<Vec<String>>;
  fn get_created_at(&self, key: &str) -> ErrFuture<Timespec>;
  fn insert(&mut self, key : &str, value: &[u8], expires_at: Option<Timespec>) -> ErrFuture<bool>;
  fn invalidate(&mut self, key: &str) -> ErrFuture<bool>;
  fn invalidate_all(&mut self) -> ErrFuture<bool>;

  fn flush(&mut self) -> ErrFuture<bool>;
  fn vacuum(&mut self) -> ErrFuture<bool>;
  fn shutdown(&mut self) -> ErrFuture<bool>;
}