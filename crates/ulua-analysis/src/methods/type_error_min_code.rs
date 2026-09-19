use crate::records::type_error::TypeError;

/// cpp `TypeError::minCode()`（Error.cpp:1273）：错误码基线。
pub const MIN_CODE: i32 = 1000;

impl TypeError {
  pub fn min_code() -> i32 {
    MIN_CODE
  }
}
