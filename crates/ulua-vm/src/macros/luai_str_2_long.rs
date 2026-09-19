use core::ffi::{c_char, c_int};

use ulua_common::strtoull_shim::rust_strtoll;

/// 全平台统一走纯 Rust `strtoll`（`ulua_common::strtoull_shim` 的
/// `parse_c_ll` 出口）：语义对齐 C 标准 + glibc/Darwin（`0x` 前缀、
/// ERANGE 饱和、`endptr` 最长有效前缀），并有完整边界测试。
/// 旧 native 分支绑 libc `strtoll` 已删，避免双实现语义漂移
/// （wasm 曾因垫片恒返 0 且不写 `endptr` 导致 `i64.fromstring` 必崩）。
///
/// # Safety
/// `s` 必须指向以 NUL 结尾的缓冲区；`endptr` 为 NULL 或可写。
#[inline]
pub unsafe fn strtoll(s: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64 {
  unsafe { rust_strtoll(s, endptr, base as u32) }
}
