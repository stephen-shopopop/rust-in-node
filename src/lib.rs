#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[cfg(test)]
mod tests {
  #[test]
  fn not_works() {
    let result = 2 + 3;
    assert_eq!(result, 4);
  }

  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
