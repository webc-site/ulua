use alloc::{boxed::Box, vec::Vec};
use core::ffi::c_int;
#[derive(Default)]
pub struct MapFixture {
  pub ptrs: Vec<Box<c_int>>,
}
