#[macro_export]
macro_rules! string {
  () => {
    String::new()
  };
  ($s:expr) => {
    String::from($s)
  };
}

#[cfg(test)]
mod tests {
  #[test]
    fn it_strs() {
      assert_eq!(string!(), String::new());
      assert_eq!(string!("e631"), String::from("e631"));
    }
}
