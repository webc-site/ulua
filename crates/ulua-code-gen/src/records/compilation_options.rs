use core::{ffi::c_char, mem::MaybeUninit, ptr::null};

use crate::records::host_ir_hooks::HostIrHooks;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct CompilationOptions {
  pub flags: u32,
  pub hooks: HostIrHooks,

  /// null-terminated array of userdata types names that might have custom lowering
  pub userdata_types: *const *const c_char,

  pub record_counters: bool,

  /// When true, random NOP sleds are inserted between blocks to
  /// make intra-function gadget offsets unpredictable.
  pub nop_padding: bool,
}

impl Default for CompilationOptions {
  fn default() -> Self {
    unsafe {
      let hooks = MaybeUninit::<HostIrHooks>::zeroed();
      Self {
        flags: 0,
        hooks: hooks.assume_init(),
        userdata_types: null(),
        record_counters: false,
        nop_padding: false,
      }
    }
  }
}
