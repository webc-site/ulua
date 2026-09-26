//! C 函数短源名 "[C]"（cpp getinfo 的 short_src 占位）。
//!
//! NUL 结尾字节串面向 `*const c_char` 契约——不引入 `CStr` 类型。

/// NUL 结尾短源名（`*const c_char` 契约用）。
pub const SHORT_SRC_C: &[u8] = b"[C]\0";
