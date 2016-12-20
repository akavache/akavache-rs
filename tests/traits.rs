extern crate akavache;
extern crate futures;
extern crate time;

#[macro_use]
extern crate quickcheck;

use akavache::memory::*;
use akavache::traits::*;
use futures::future::*;

quickcheck! {

fn should_be_able_to_get_and_insert_blobs(input: Vec<u8>) -> bool {
  if input.len() == 0 { return true; }

  let mut fixture = InMemoryBlobCache::new();

  let result = fixture.insert("foo", &input, None).wait().unwrap();
  if !result { return false; }

  let vec = fixture.get("foo").wait().unwrap();

  if input.len() != vec.len() { return false }
  for pair in input.iter().zip(vec.iter()) {
    if pair.0 != pair.1 { return false; }
  }

  true
}

}
