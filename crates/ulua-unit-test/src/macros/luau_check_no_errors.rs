#[macro_export]
macro_rules! LUAU_CHECK_NO_ERRORS {
  ($result:expr) => {
    $crate::LUAU_CHECK_ERROR_COUNT!(0, $result);
  };
}

pub use LUAU_CHECK_NO_ERRORS;
