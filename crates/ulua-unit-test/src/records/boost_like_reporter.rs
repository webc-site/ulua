use core::ffi::c_void;
#[derive(Debug)]
#[repr(C)]
pub struct BoostLikeReporter {
  pub current_test: *const c_void,
}
