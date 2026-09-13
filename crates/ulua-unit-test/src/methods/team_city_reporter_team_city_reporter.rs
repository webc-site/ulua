use core::{ffi::c_void, ptr::null};

use crate::records::team_city_reporter::TeamCityReporter;
impl TeamCityReporter {
  pub fn new(_in: *const c_void) -> Self {
    Self {
      current_test: null(),
    }
  }
}

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn team_city_reporter_team_city_reporter(
  this: *mut TeamCityReporter,
  _in: *const c_void,
) {
  unsafe {
    (*this).current_test = null();
  }
}
