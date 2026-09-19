use core::ffi::CStr;

/// 错误处理中出错消息（cpp ERRERRMSG 宏同款，NUL 结尾）
pub const LUA_ERRERRMSG: &CStr = c"error in error handling";
