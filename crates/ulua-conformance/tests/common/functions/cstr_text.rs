//! 测试「文本断言层」读取 C 字符串的唯一收口（review.md §10）。
//!
//! 通过 C ABI 调用 `lua_*` 取回 `*const c_char` 是合法的；非法的是把这些裸指针
//! 就地 `CStr::from_ptr(..).to_string_lossy()` / `.to_bytes()` 散落到各用例里做比对
//! 或渲染。本模块把「取回来转文本」这一步收口到 ulua-common 的 `c_str` 门面
//! （[`cstr_cow`] / [`cstr_bytes`]）之上，用例只经这里语义化的出口，门面外不再出现
//! `CStr::from_ptr`。
//!
//! [`cstr_cow`]: ulua_common::functions::c_str::cstr_cow
//! [`cstr_bytes`]: ulua_common::functions::c_str::cstr_bytes

use alloc::{borrow::Cow, string::String};
use core::ffi::{c_char, c_int};

use ulua_common::functions::c_str::{cstr_bytes, cstr_cow};
use ulua_vm::{macros::lua_tostring::lua_tostring, records::lua_state::LuaState};

/// 取回一段 C 文本做断言或错误消息渲染（UTF-8 宽容解码，null 按空串处理）。
///
/// 与被替换的 `CStr::from_ptr(p).to_string_lossy()` 逐字等价，仅把 `from_ptr` 收口到
/// ulua-common 门面一处。比较对象为 ASCII 字面量时，其结果与旧的
/// `.to_str().unwrap_or("")`（严格解码 + 空回退）也一致：ASCII 下两者恒同。
///
/// # Safety
/// `p` 为 null，或指向在返回值生命周期 `'a` 内持续有效的 NUL 结尾缓冲。
#[inline]
pub unsafe fn cstr_text<'a>(p: *const c_char) -> Cow<'a, str> {
  // Safety: 前置条件与 `cstr_cow` 完全一致（null / NUL 结尾 + 存活期 `'a`）。
  unsafe { cstr_cow(p) }
}

/// 取回 C 串的原始字节做逐字节断言（免 UTF-8 解码，非 UTF-8 / 内嵌字节保真）。
///
/// 与被替换的 `CStr::from_ptr(p).to_bytes()` 逐字等价。
///
/// # Safety
/// 同 [`cstr_text`]。
#[inline]
pub unsafe fn cstr_raw<'a>(p: *const c_char) -> &'a [u8] {
  // Safety: 前置条件与 `cstr_bytes` 完全一致。
  unsafe { cstr_bytes(p) }
}

/// `lua_tostring!(l, idx)` + lossy 收口：读取栈 `idx` 处字符串的文本。栈值不可转成
/// 字符串时 `lua_tolstring` 返回 null，按空串处理。用于错误消息 / 结果串的文本断言。
///
/// # Safety
/// `l` 为存活的 `LuaState`，`idx` 为合法栈索引；返回的借用串在下一次改栈前有效，
/// 调用方须即时消费（比对 / 格式化）。
#[inline]
pub unsafe fn lua_tostring_text<'a>(l: *mut LuaState, idx: c_int) -> Cow<'a, str> {
  // Safety: `lua_tostring!` 返回 null 或本帧栈槽保活的 NUL 结尾缓冲，交 `cstr_cow`。
  unsafe { cstr_cow(lua_tostring!(l, idx)) }
}

/// 空指针兜底的诊断文本：专供「断言失败前打印定位信息」路径（`lua_tostring` /
/// `lua_debugtrace` 可能返回 null，直接解引用会吞掉真正的失败原因）。null 译为
/// `"<null>"` 以区分「无值」与「空串」；非 null 按 lossy 渲染。仅人读，不参与断言。
///
/// # Safety
/// `p` 为 null 或以 NUL 结尾的合法 C 字符串。
#[inline]
pub unsafe fn diagnostic_text(p: *const c_char) -> String {
  if p.is_null() {
    String::from("<null>")
  } else {
    // Safety: 上一分支已排除 null，`p` 为 NUL 结尾串。
    unsafe { cstr_text(p) }.into_owned()
  }
}
