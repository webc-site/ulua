extern crate alloc;

use alloc::string::String;
use core::ffi::c_void;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Node {
  pub ptr: *mut c_void,
  pub tag: u8,
  pub memcat: u8,
  pub size: usize,
  pub name: String,
}
