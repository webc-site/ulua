//! `*const c_char` 读取的 crate 内 safe 门面（仿 `ulua-repl-cli::functions::state_ref`
//! 先例：把「调用方承载裸参契约」下沉到 crate 内部）。门面本身 `pub(crate)` safe `fn`，
//! 非导出即免检，故 `clippy::not_unsafe_ptr_arg_deref` 不在其上触发；导出面
//! （如 `lua_setlightuserdataname`）经本门面消费 C 串裸参后，裸参只入 safe 被调实参位，
//! lint 触发消亡。不改 `ulua_common::functions::c_str::cstr_bytes` 本体，不在
//! ulua-common 加面。
//!
//! # Safety（模块级契约，全部调用点共同依赖）
//!
//! 入参 `p` 须为 NUL 结尾 C 串的可读指针或 null（`cstr_bytes` 对 null 返空切片），
//! 且缓冲在调用期间存活、首个 NUL 前全程可读。调用点（当前仅
//! `lua_setlightuserdataname` 的 c-API 边界）须已按其 `# Safety` 文档保证传入 C 串
//! 满足上述前提，再经本门面取首个 NUL 前字节窗。

use core::ffi::c_char;

use crate::functions::cstr_bytes;

/// [`cstr_bytes`] 的 crate 内 safe 转达：签名非 `unsafe`、可见性 `pub(crate)`，
/// 把 `unsafe` 收拢在本门面单点（契约见模块头），导出面经其消费裸 `*const c_char`
/// 参数不再触发 `not_unsafe_ptr_arg_deref`。行为与直接调用 `cstr_bytes` 逐位等价。
#[inline]
pub(crate) fn cstr_bytes_ref<'a>(p: *const c_char) -> &'a [u8] {
  // SAFETY: 模块级契约保证 `p` 为 NUL 结尾 C 串的可读指针或 null，`cstr_bytes` 的
  // NUL 扫描必在其界内终止（null 短路空切片）。unsafe 单点收拢于此。
  unsafe { cstr_bytes(p) }
}
