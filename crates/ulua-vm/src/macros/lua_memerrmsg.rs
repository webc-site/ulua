use core::ffi::CStr;

/// 内存不足错误消息（cpp MEMERRMSG 宏同款，NUL 结尾）
pub const LUA_MEMERRMSG: &CStr = c"not enough memory";
