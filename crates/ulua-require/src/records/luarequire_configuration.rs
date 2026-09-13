use core::ffi::{c_char, c_void};

use crate::enums::{
  luarequire_config_status::luarequire_ConfigStatus,
  luarequire_navigate_result::luarequire_NavigateResult,
  luarequire_write_result::luarequire_WriteResult,
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct luarequire_Configuration {
  pub is_require_allowed: Option<
    unsafe extern "C-unwind" fn(
      l: *mut c_void,
      ctx: *mut c_void,
      requirer_chunkname: *const c_char,
    ) -> bool,
  >,
  pub reset: Option<
    unsafe extern "C-unwind" fn(
      l: *mut c_void,
      ctx: *mut c_void,
      requirer_chunkname: *const c_char,
    ) -> luarequire_NavigateResult,
  >,
  pub jump_to_alias: Option<
    unsafe extern "C-unwind" fn(
      l: *mut c_void,
      ctx: *mut c_void,
      path: *const c_char,
    ) -> luarequire_NavigateResult,
  >,
  pub to_alias_override: Option<
    unsafe extern "C-unwind" fn(
      l: *mut c_void,
      ctx: *mut c_void,
      alias_unprefixed: *const c_char,
    ) -> luarequire_NavigateResult,
  >,
  pub to_alias_fallback: Option<
    unsafe extern "C-unwind" fn(
      l: *mut c_void,
      ctx: *mut c_void,
      alias_unprefixed: *const c_char,
    ) -> luarequire_NavigateResult,
  >,
  pub to_parent: Option<
    unsafe extern "C-unwind" fn(l: *mut c_void, ctx: *mut c_void) -> luarequire_NavigateResult,
  >,
  pub to_child: Option<
    unsafe extern "C-unwind" fn(
      l: *mut c_void,
      ctx: *mut c_void,
      name: *const c_char,
    ) -> luarequire_NavigateResult,
  >,
  pub is_module_present:
    Option<unsafe extern "C-unwind" fn(l: *mut c_void, ctx: *mut c_void) -> bool>,
  pub get_chunkname: Option<
    unsafe extern "C-unwind" fn(
      l: *mut c_void,
      ctx: *mut c_void,
      buffer: *mut c_char,
      buffer_size: usize,
      size_out: *mut usize,
    ) -> luarequire_WriteResult,
  >,
  pub get_loadname: Option<
    unsafe extern "C-unwind" fn(
      l: *mut c_void,
      ctx: *mut c_void,
      buffer: *mut c_char,
      buffer_size: usize,
      size_out: *mut usize,
    ) -> luarequire_WriteResult,
  >,
  pub get_cache_key: Option<
    unsafe extern "C-unwind" fn(
      l: *mut c_void,
      ctx: *mut c_void,
      buffer: *mut c_char,
      buffer_size: usize,
      size_out: *mut usize,
    ) -> luarequire_WriteResult,
  >,
  pub get_config_status: Option<
    unsafe extern "C-unwind" fn(l: *mut c_void, ctx: *mut c_void) -> luarequire_ConfigStatus,
  >,
  pub get_alias: Option<
    unsafe extern "C-unwind" fn(
      l: *mut c_void,
      ctx: *mut c_void,
      alias: *const c_char,
      buffer: *mut c_char,
      buffer_size: usize,
      size_out: *mut usize,
    ) -> luarequire_WriteResult,
  >,
  pub get_config: Option<
    unsafe extern "C-unwind" fn(
      l: *mut c_void,
      ctx: *mut c_void,
      buffer: *mut c_char,
      buffer_size: usize,
      size_out: *mut usize,
    ) -> luarequire_WriteResult,
  >,
  pub get_luau_config_timeout:
    Option<unsafe extern "C-unwind" fn(l: *mut c_void, ctx: *mut c_void) -> i32>,
  // `load` may raise a Lua error (e.g. when the required module fails at
  // runtime). With the panic-based `luaD_throw`, that error unwinds out of the
  // callback, so the boundary must permit unwinding (`C-unwind`).
  pub load: Option<
    unsafe extern "C-unwind" fn(
      l: *mut c_void,
      ctx: *mut c_void,
      path: *const c_char,
      chunkname: *const c_char,
      loadname: *const c_char,
    ) -> i32,
  >,
}

/// 配置初始化回调（对应 C++ `luarequire_Configuration_init`）。
pub type LuarequireConfigurationInit =
  Option<unsafe extern "C-unwind" fn(config: *mut luarequire_Configuration)>;
