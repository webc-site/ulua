use core::ffi::c_void;
#[derive(Debug)]
#[repr(C)]
pub struct TeamCityReporter {
  pub current_test: *const c_void,
}
