#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[cfg(test)]
mod plus_100 {
  #[test]
  fn not_works() {
    let result = super::plus_100(1);

    assert_ne!(result, 100);
  }

  #[test]
  fn it_works() {
    let result = super::plus_100(1);

    assert_eq!(result, 101);
  }
}
