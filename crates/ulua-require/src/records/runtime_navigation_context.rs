use alloc::ffi::CString;
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

pub struct RuntimeNavigationContext {
  pub(crate) config: *mut luarequire_Configuration,
  pub(crate) l: *mut c_void,
  pub(crate) ctx: *mut c_void,
  pub(crate) requirer_chunkname: CString,
  pub(crate) timer: RuntimeLuauConfigTimer,
}
