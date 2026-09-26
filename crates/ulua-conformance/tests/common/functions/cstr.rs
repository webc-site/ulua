//! NUL 结尾字节串字面量（`b"name\0"`）→ `*const c_char` 的唯一收口。
//!
//! review.md §10：C 字符串字面量（c 前缀）仅豁免于 C ABI 实现层（ulua-vm/ulua-capi
//! 的 `lua_*.rs`/`*_shim.rs`），测试代码一律用 `b"..\0"` 字节串，并只在调用
//! `*const c_char` 契约 API 的收口点转成裸指针——收口集中在本函数，不散落
//! `.as_ptr()` 逐点 cast。

use core::ffi::c_char;

/// `bytes` 须为以 NUL 结尾的静态字节串（`b"name\0"` 字面量即满足）；返回指针
/// 随 `'static` 缓冲区存活，满足 C 侧对参数存活期的要求。
pub fn cstr(bytes: &'static [u8]) -> *const c_char {
  bytes.as_ptr().cast()
}
