use core::{ffi::c_void, mem::zeroed};

use crate::records::gc_object::GcObject;

#[repr(C)]
#[derive(Copy, Clone)]
pub union Value {
  pub gc: *mut GcObject,
  pub p: *mut c_void,
  pub n: f64,
  pub b: i32,
  pub l: i64,
  pub v: [f32; 2],
}

impl Default for Value {
  /// C aggregate zero-initialization (`Value v = {0}`): the all-zero bit
  /// pattern is valid for every member of the union.
  fn default() -> Self {
    // Safety: 全零位模式对 union 每个成员（指针=空、数值=0）均为合法值，等价 C 聚合零初始化
    unsafe { zeroed() }
  }
}
