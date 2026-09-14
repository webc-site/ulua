#[macro_export]
macro_rules! LUAU_REQUIRE_ERRORS {
  ($result:expr) => {
    let r = $result;
    self.validate_errors(&r.errors);
    assert!(!r.errors.is_empty());
  };
}

pub use LUAU_REQUIRE_ERRORS;
