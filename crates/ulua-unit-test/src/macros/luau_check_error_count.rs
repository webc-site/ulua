#[macro_export]
macro_rules! LUAU_CHECK_ERROR_COUNT {
  ($count:expr, $result:expr) => {
    let r = $result;
    $crate::validate_errors(&r.errors);
    $crate::CHECK_MESSAGE!($count == r.errors.len(), "{}", $crate::get_errors(&r));
  };
}

pub use LUAU_CHECK_ERROR_COUNT;
