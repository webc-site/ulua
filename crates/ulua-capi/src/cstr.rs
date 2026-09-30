//! FFI 边界内的 C 字符串读取辅助（单源，供各表示适配壳复用）。
//! NUL 结尾缓冲 → `&str` 的转换脚手架只在本模块书写一次，非 UTF-8 的兜底语义
//! 在此统一：返回空串，使调用方走「名称不匹配 / 错误消息退化」路径，与 oracle
//! `strcmp` 失败同向。仅 crate 内可见，不构成对外 API（禁二次导出）。

use core::ffi::{CStr, c_char};

/// 把 NUL 结尾的 C 串缓冲重建为借用期不超过本次 FFI 调用的 `&str`；
/// 非 UTF-8 字节兜底为空串。
///
/// # Safety
/// `ptr` 必须指向 NUL 结尾的只读串缓冲，对齐且在本次调用期间存活（Lua/C API
/// 调用方约定，由各壳的 `/// # Safety` 契约声明）。
pub(crate) unsafe fn to_str_or_empty<'a>(ptr: *const c_char) -> &'a str {
  // Safety: 调用方（各壳）的 `# Safety` 契约经本函数 `# Safety` 透传，保证
  // `ptr` 为 NUL 结尾存活缓冲，`from_ptr` 重建借用合法；借用期上界由壳调用
  // 帧约束，不外泄。
  unsafe { CStr::from_ptr(ptr) }.to_str().unwrap_or("")
}
