pub fn add(input: i32) -> i32 {
  let t = 1;
  input + t
}

#[cfg(test)]
mod add {
  #[test]
  fn it_works() {
    let result = super::add(1);

    assert_eq!(result, 2);
  }
}
