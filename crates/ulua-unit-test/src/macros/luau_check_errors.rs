#[macro_export]
macro_rules! LUAU_CHECK_ERRORS {
  ($result:expr) => {
    let r = $result;
    self.validate_errors(&r.errors);
    assert!(!r.errors.is_empty());
  };
}

pub use LUAU_CHECK_ERRORS;
