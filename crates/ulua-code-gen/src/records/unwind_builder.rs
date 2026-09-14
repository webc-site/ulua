use core::{ffi::c_void, ptr::null};

use crate::enums::arch::Arch;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UnwindBuilder {
  _vtable: *const c_void,
}

impl UnwindBuilder {
  pub const X64: Arch = Arch::X64;
  pub const A64: Arch = Arch::A64;
}

pub type UnwindBuilderArch = Arch;

impl Default for UnwindBuilder {
  fn default() -> Self {
    Self { _vtable: null() }
  }
}
