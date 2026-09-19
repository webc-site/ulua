use core::ffi::c_void;

use crate::records::{
  luarequire_configuration::luarequire_Configuration,
  runtime_luau_config_timer::RuntimeLuauConfigTimer,
};

/// 标识符类字符串（chunkname/loadname/cache_key/alias）的初始缓冲区大小
/// （对应 C++ `initalIdentifierBufferSize`）。
pub(crate) const INITIAL_IDENTIFIER_BUFFER_SIZE: usize = 64;

/// 配置文件内容的初始缓冲区大小（对应 C++ `initalFileBufferSize`）。
pub(crate) const INITIAL_FILE_BUFFER_SIZE: usize = 1024;

pub struct RuntimeNavigationContext<'ctx> {
  pub(crate) config: *mut luarequire_Configuration,
  pub(crate) l: *mut c_void,
  pub(crate) ctx: *mut c_void,
  /// 借用调用方提供的 requirer chunkname 字节串（Lua 字符串非 UTF-8），
  /// 避免每次 require 分配；仅经 `with_c_str` 在真 FFI 边界补 NUL
  pub(crate) requirer_chunkname: &'ctx [u8],
  pub(crate) timer: RuntimeLuauConfigTimer,
}
