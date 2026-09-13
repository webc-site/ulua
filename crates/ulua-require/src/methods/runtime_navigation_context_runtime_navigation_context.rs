use alloc::ffi::CString;
use core::ffi::c_void;
use std::time::Instant;

use crate::records::{
  luarequire_configuration::luarequire_Configuration,
  runtime_luau_config_timer::RuntimeLuauConfigTimer,
  runtime_navigation_context::RuntimeNavigationContext,
};

impl RuntimeNavigationContext {
  /// # Safety
  ///
  /// `config` must be null or point to a valid `luarequire_Configuration`.
  /// `l` must be a valid pointer to a live `lua_State`.
  pub unsafe fn new(
    config: *mut luarequire_Configuration,
    l: *mut c_void,
    ctx: *mut c_void,
    requirer_chunkname: CString,
  ) -> Self {
    Self {
      config,
      l,
      ctx,
      requirer_chunkname,
      timer: RuntimeLuauConfigTimer {
        start_time: Instant::now(),
        timeout_duration: None,
      },
    }
  }
}
