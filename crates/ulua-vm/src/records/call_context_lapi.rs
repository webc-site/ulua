use core::ffi::c_int;
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub(crate) struct CallContext {
  pub(crate) size: c_int,
}
