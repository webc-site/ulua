#[macro_export]
macro_rules! LUAU_REQUIRE_ERROR_COUNT {
  ($count:expr, $result:expr) => {
    let r = $result;
    self.validate_errors(&r.errors);
    $crate::REQUIRE_MESSAGE!($count == r.errors.len(), "{}", self.get_errors(&r));
  };
}

pub use LUAU_REQUIRE_ERROR_COUNT;
