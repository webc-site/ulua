use core::ffi::{c_char, c_int};

#[cfg(not(target_arch = "wasm32"))]
unsafe extern "C" {
  pub fn strtoll(s: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64;
}

// wasm 无 libc `strtoll`：转发纯 Rust 实现（i64 饱和 + endptr 语义见
// `ulua_common::strtoull_shim`）。旧垫片恒返 0 且不写 `endptr`，`luaO_str2l`
// 随后解引用 null `endptr`，`i64.fromstring` 在 wasm 上必崩。
#[cfg(target_arch = "wasm32")]
use ulua_common::strtoull_shim::rust_strtoll;

/// # Safety
/// `s` 必须指向以 NUL 结尾的缓冲区；`endptr` 为 NULL 或可写。
#[cfg(target_arch = "wasm32")]
#[inline]
pub unsafe fn strtoll(s: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64 {
  unsafe { rust_strtoll(s, endptr, base as u32) }
}
