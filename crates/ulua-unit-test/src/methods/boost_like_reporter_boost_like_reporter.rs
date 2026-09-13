use core::{ffi::c_void, ptr::null};

use crate::records::boost_like_reporter::BoostLikeReporter;
impl BoostLikeReporter {
  pub fn new(_in: *const c_void) -> Self {
    Self {
      current_test: null(),
    }
  }
}

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn boost_like_reporter_boost_like_reporter(
  this: *mut BoostLikeReporter,
  _in_: *const c_void,
) {
  unsafe {
    (*this).current_test = null();
  }
}
