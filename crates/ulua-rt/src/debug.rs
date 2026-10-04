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

use crate::{
  state::{Lua, StateView},
  sys::*,
};

/// `lua_getinfo` 的选项串：`n`（名字）+ `s`（源/类型）+ `l`（当前行）。
/// review.md §10 后为原生选项字节窗（无终止 NUL），整窗即模板。
const GETINFO_NSL: &[u8] = b"nsl";

/// VM 回填的原生字节窗 → Rust 串（lossy 解码，与旧 `cstr_cow` 读面逐字节等值：
/// 旧面 null→`None`、非 null→NUL 扫描前缀 lossy；新窗在写端已截 NUL）。
/// [`Lua::inspect_stack`] 与 `Function::info` 共用。
pub(crate) fn debug_str(bytes: Option<&[u8]>) -> Option<String> {
  bytes.map(|b| String::from_utf8_lossy(b).into_owned())
}

/// `lua_getinfo` 的 safe 门面（本 crate 唯一 getinfo 边界）：以默认 `LuaDebug`
/// 作 out 参数按 `options` 模板回填，解析不到（VM 返回 ok==0）时给 `None`。
///
/// 调用序契约（正确性，非内存安全）：`state` 存活且由当前线程驱动；`level` 有效
/// （>=0 计帧数、-1 指栈顶——本仓库 Luau 的 `lua_getinfo` 无 5.x 的 `">"` 弹栈
/// 约定）；`options` 为选项字节窗且不含 `f` 选项（含 `f` 会向栈压值，本门面不做
/// 栈配平）。
pub(crate) fn get_info(state: StateView<'_>, level: i32, options: &[u8]) -> Option<LuaDebug> {
  let mut ar: LuaDebug = LuaDebug::default();
  // Safety: `state` 存活且由当前线程驱动（调用点句柄契约）；`&mut ar` 指向本帧
  // 独占可写的局部记录，VM 按 `options` 直填原生字段（串体借闭包→Proto 存活，
  // 本帧内由调用方即时拷成 owned 值）。
  let ok = unsafe { lua_getinfo(state.as_mut_ptr(), level, options, &mut ar) };
  (ok != 0).then_some(ar)
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
    // `get_info`（safe 门面）收口默认初值 + `lua_getinfo` out 参数两步；
    // `GETINFO_NSL` 是原生选项字节窗（无终止 NUL）。ok==0 时不读 `ar`。
    let ar = get_info(state, level as i32, GETINFO_NSL)?;
    // 以下只读 `ar` 的原生回填字段（字节窗比较 + lossy 拷串），不再触任何
    // C 边界，故无 unsafe。`what` 模板字节为 VM 静态字面量 `b"Lua"`/`b"C"`；
    // `b"main"` 分支承接 mlua 语义面（VM 从不写该值，行为与旧扫描读一致）。
    let what = match ar.what {
      Some(b"Lua") => DebugWhat::Lua,
      Some(b"main") => DebugWhat::Main,
      Some(b"C") => DebugWhat::C,
      _ => DebugWhat::Unknown,
    };
    let current_line = (ar.currentline >= 0).then_some(ar.currentline as i64);
    let line_defined = (ar.linedefined > 0).then_some(ar.linedefined as i64);
    Some(Debug {
      name: debug_str(ar.name),
      what,
      source: debug_str(ar.source),
      short_src: debug_str(Some(ar.short_src.bytes())),
      current_line,
      line_defined,
    })
  }
}
