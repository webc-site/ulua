#[macro_export]
macro_rules! NONSTRICT_REQUIRE_CHECKED_ERR {
  ($pos:expr, $name:expr, $result:expr) => {
    let mut err_index = 0;
    $crate::macros::nonstrict_require_err_at_pos::NONSTRICT_REQUIRE_ERR_AT_POS!(
      $pos, $result, err_index
    );
    let err = unsafe {
      $crate::functions::get::get::<$crate::records::error::CheckedFunctionCallError>(
        &$result.errors[err_index],
      )
    };
    assert!(err.is_some());
    let err_ptr = err.unwrap();
    assert_eq!(err_ptr.checked_function_name, $name);
  };
}

pub use NONSTRICT_REQUIRE_CHECKED_ERR;
