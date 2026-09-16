#[macro_export]
macro_rules! LUAU_REQUIRE_NO_ERRORS {
  ($result:expr) => {
    $crate::LUAU_REQUIRE_ERROR_COUNT!(0, $result);
  };
}

pub use LUAU_REQUIRE_NO_ERRORS;
