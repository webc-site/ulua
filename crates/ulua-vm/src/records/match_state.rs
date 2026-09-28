use core::ptr::null_mut;

use crate::{macros::lua_maxcaptures::LUA_MAXCAPTURES, records::lua_state::LuaState};

/// cpp `lstrlib.cpp` 的 `MatchState`（lstrlib.cpp:182）：b10 起 s/p 串游标全部索引化，
/// rev.ptr-r03 起两个串游标的载体直接是 Lua 串 payload 的借用切片。
///
/// 原 `src_init`/`src_end` 指针对 → `src: &'a [u8]`，源游标为相对源串的 `usize`
/// 偏移，`偏移 == src.len()` 即原 `s == src_end` 哨兵；原 `p_end` 哨兵 →
/// `pat: &'a [u8]`，pattern 游标偏移 `== pat.len()` 即原 `p == p_end`；捕获
/// `capture[].init` 由源串指针改为源偏移（cpp `init - src_init` 的差值语义即偏移本身）。
///
/// 生命周期 `'a` 即 cpp 侧「实参串在本次匹配调用全程存活」的 `luaL_checklstring`
/// 契约（由 [`crate::functions::prepstate::prepstate`] 的调用点建立），结构体内
/// 不再有裸指针，故无任何 `unsafe`；字节/窗口读取统一走下方安全门面。
#[derive(Debug, Clone, Copy)]
pub struct MatchState<'a> {
  pub(crate) matchdepth: i32,
  /// 源串 payload（cpp `src_init..src_end`；终止 NUL 不在切片内，尾字节读取由
  /// [`MatchState::src_byte`] 按 cpp 同点位语义折算为 0）。
  pub(crate) src: &'a [u8],
  /// pattern payload（cpp `p + 1..ms->p_end` 即锚定字符被跳过后的区间，同上）。
  pub(crate) pat: &'a [u8],
  pub(crate) l: *mut LuaState,
  pub(crate) level: i32,
  pub(crate) capture: [Capture; CAPTURES],
}

/// cpp `MatchState.capture[]` 的元素（匿名 struct）：`init` 为源串偏移，`len`
/// 仍按 cpp 语义存捕获长度或 `CAP_UNFINISHED`/`CAP_POSITION` 哨兵值。
#[derive(Debug, Clone, Copy)]
pub struct Capture {
  pub(crate) init: usize,
  pub(crate) len: isize,
}

/// cpp `lstrlib.cpp` `LUA_MAXCAPTURES`（lstrlib.cpp:180，宏 `lua_maxcaptures`）：
/// 捕获槽上限。
const CAPTURES: usize = LUA_MAXCAPTURES as usize;

/// 终止 NUL 的字节值：cpp 在 `s == src_end`/`p == p_end` 处直读 `*s`/`*p`
/// 取到的恒为该值（Lua 串 payload 后恒有 `\0`）。
const NUL: u8 = 0;

/// 借用区上的 `[off, off + len)` 窗口：界内与 cpp 指针区间逐字节一致，界外一律
/// 截断（cpp 的界外读在此收敛为有界空读，不再是 UB）。
#[inline]
fn window(bytes: &[u8], off: usize, len: usize) -> &[u8] {
  let start = off.min(bytes.len());
  let end = off.saturating_add(len).min(bytes.len());
  &bytes[start..end]
}

impl<'a> MatchState<'a> {
  /// 取源串偏移 `off` 处字节。`off == src.len()` 返回终止 NUL，对应 cpp 在
  /// `s == src_end` 处读 `*s` 的同点位行为（`off > src.len()` 恒不可达，同样
  /// 取 0 保证全函数无 UB）。cpp `lstrlib.cpp` `match`/`singlematch`。
  #[inline]
  pub(crate) fn src_byte(&self, off: usize) -> u8 {
    self.src.get(off).copied().unwrap_or(NUL)
  }

  /// 取 pattern 偏移 `off` 处字节，语义同 [`Self::src_byte`]（`off == pat.len()`
  /// 即 cpp 读模式串终止 NUL：空 pattern/尾字节判断的同点位）。
  #[inline]
  pub(crate) fn pat_byte(&self, off: usize) -> u8 {
    self.pat.get(off).copied().unwrap_or(NUL)
  }

  /// 源串 `[off, off + len)` 子串：捕获比对与 C-API 边界指针重建均按切片消费。
  #[inline]
  pub(crate) fn src_slice(&self, off: usize, len: usize) -> &'a [u8] {
    window(self.src, off, len)
  }

  /// pattern `[off, off + len)` 子串（字符类扫描窗口，`len = ep - p`，`ep` 右开界）。
  #[inline]
  pub(crate) fn pat_slice(&self, off: usize, len: usize) -> &'a [u8] {
    window(self.pat, off, len)
  }
}

impl Default for MatchState<'_> {
  fn default() -> Self {
    Self {
      matchdepth: 0,
      src: &[],
      pat: &[],
      l: null_mut(),
      level: 0,
      capture: [Capture { init: 0, len: 0 }; CAPTURES],
    }
  }
}
