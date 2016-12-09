use std;
use futures;

use time::Timespec;

type ErrFuture<T> = futures::Future<Item = T, Error = std::io::Error>;

pub trait BlobCache {
  fn get(key: &str) -> ErrFuture<Vec<u8>>;
  fn get_all_keys() -> ErrFuture<Vec<String>>;
  fn get_created_at(key: &str, absoluteExpiration: Option<Timespec>) -> ErrFuture<Timespec>;
  fn insert(key : &str, value: &[u8]) -> ErrFuture<bool>;
  fn invalidate(key: &str) -> ErrFuture<bool>;
  fn invalidate_all() -> ErrFuture<bool>;

  fn flush() -> ErrFuture<bool>;
  fn vacuum() -> ErrFuture<bool>;
  fn shutdown() -> ErrFuture<bool>;
}