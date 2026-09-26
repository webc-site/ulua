//! Stack inspection. Mirrors the Luau-feasible subset of `mlua::Lua::inspect_stack`
//! and `mlua::debug::Debug`.
//!
//! ## What Luau can back
//!
//! Luau's debug model is **not** the Lua 5.x line/count hook. It exposes
//! `lua_getinfo(l, level, what, ar)` for activation records, plus
//! `lua_singlestep` and the interrupt callback for stepping. We surface the *informational* part —
//! resolving a stack level into a [`Debug`] record (current line, source, name,
//! what kind of function) — which maps cleanly onto `lua_getinfo`.
//!
//! ## What is deferred (and why)
//!
//! The full `mlua::Lua::set_hook(HookTriggers, ...)` API (per-line / per-N-
//! instruction / on-call / on-return hooks with a `Debug` event) is a Lua 5.x
//! construct. Luau has no equivalent multiplexed hook: it has a *single* global
//! interrupt callback (see [`Lua::set_interrupt`](crate::Lua::set_interrupt))
//! and `lua_singlestep`. mlua itself gates `tests/hooks.rs` and `tests/debug.rs`
//! behind `#![cfg(not(feature = "luau"))]` for exactly this reason. We therefore
//! do **not** fake a 5.x hook surface; the interrupt API is the Luau-native
//! analog and is implemented separately.

use core::{ffi::c_char, mem::zeroed};

use ulua_common::functions::c_str::cstr_cow;

use crate::{state::Lua, sys::*};

/// `lua_getinfo` 的选项串：`n`（名字）+ `s`（源/类型）+ `l`（当前行）。
/// 静态 NUL 结尾字节串，收口点交给 `*const c_char` 契约 API。
const GETINFO_NSL: &[u8] = b"nsl\0";

/// 把 VM 报告的 `*const c_char` 调试字段转成 Rust 字符串（null -> `None`）。
/// [`Lua::inspect_stack`] 与 `Function::info` 共用。safe 收口：入参只可能是
/// `lua_Debug` 回填字段（NUL 结尾 VM 串）或未回填的 null，两分支就地处理，
/// 无调用方前置条件。
pub(crate) fn debug_cstr(p: *const c_char) -> Option<String> {
  if p.is_null() {
    None
  } else {
    // Safety: `p` 非 null 且指向 VM 内部存活、NUL 结尾的字符串（上方判空）。
    Some(unsafe { cstr_cow(p) }.into_owned())
  }
}

/// What kind of function an activation record refers to. Mirrors the relevant
/// part of `mlua::debug::DebugSource::what`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugWhat {
  /// A Lua function.
  Lua,
  /// The main chunk.
  Main,
  /// A C / Rust (native) function.
  C,
  /// Unknown / unavailable.
  Unknown,
}

/// A snapshot of one activation record, resolved from a stack level via
/// `lua_getinfo`. Mirrors the informational subset of `mlua::debug::Debug`.
#[derive(Debug, Clone)]
pub struct Debug {
  name: Option<String>,
  what: DebugWhat,
  source: Option<String>,
  short_src: Option<String>,
  current_line: Option<i64>,
  line_defined: Option<i64>,
}

impl Debug {
  /// The function's name, if known (`(n)`).
  pub fn name(&self) -> Option<&str> {
    self.name.as_deref()
  }

  /// What kind of function this record refers to (`(s)`).
  pub fn what(&self) -> DebugWhat {
    self.what
  }

  /// The chunk source (`(s)`).
  pub fn source(&self) -> Option<&str> {
    self.source.as_deref()
  }

  /// A short, human-readable source description (`(s)`).
  pub fn short_src(&self) -> Option<&str> {
    self.short_src.as_deref()
  }

  /// The currently executing line (`(l)`), if available.
  pub fn current_line(&self) -> Option<i64> {
    self.current_line
  }

  /// The line where the function was defined (`(s)`).
  pub fn line_defined(&self) -> Option<i64> {
    self.line_defined
  }
}

impl Lua {
  /// Inspect the activation record `level` frames up the call stack (0 = the
  /// currently running function). Returns `None` if there is no function at
  /// that level. Mirrors the Luau-feasible part of `mlua::Lua::inspect_stack`.
  pub fn inspect_stack(&self, level: usize) -> Option<Debug> {
    let state = self.state();
    // `ar` 是本地 `#[repr(C)]` 结构，全字段为整数/裸指针，`zeroed()` 对其每个
    // 字段都是合法位模式（null 指针与 0 整数均可表示）。
    // Safety: `ar` 每个字段皆整数/裸指针，全零位模式合法。
    let mut ar: LuaDebug = unsafe { zeroed() };
    // Safety: `state` 为存活 VM 主状态（`self.state()`）；`GETINFO_NSL` 是静态
    // NUL 结尾字节串（`lua_getinfo` 的 `*const c_char` 收口点）；`&mut ar`
    // 指向本帧对齐存活的局部；`lua_getinfo` 按 ulua 的 Rust 入口约定只会向该 out
    // 参数写非空或保持 null 的 `*const c_char` 内部字符串指针（`debug_cstr` 已对
    // null 判空）。ok==0 时下面不读 `ar` 内容。
    let ok = unsafe { lua_getinfo(state, level as c_int, GETINFO_NSL.as_ptr().cast(), &mut ar) };
    if ok == 0 {
      return None;
    }
    // 以下仅按 `#[repr(C)]` 读 `ar` 各标量字段（`debug_cstr` 是带 null 判据的
    // safe fn），不再触任何 C 边界，故无 unsafe。
    let what_str = debug_cstr(ar.what).unwrap_or_default();
    let what = match what_str.as_str() {
      "Lua" => DebugWhat::Lua,
      "main" => DebugWhat::Main,
      "C" => DebugWhat::C,
      _ => DebugWhat::Unknown,
    };
    let current_line = if ar.currentline >= 0 {
      Some(ar.currentline as i64)
    } else {
      None
    };
    let line_defined = if ar.linedefined > 0 {
      Some(ar.linedefined as i64)
    } else {
      None
    };
    Some(Debug {
      name: debug_cstr(ar.name),
      what,
      source: debug_cstr(ar.source),
      short_src: debug_cstr(ar.short_src),
      current_line,
      line_defined,
    })
  }
}
