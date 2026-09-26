//! Source: `VM/include/lualib.h` (lualib.h:86-98, hand-ported)

use core::{ffi::c_char, ptr::null_mut};

use crate::records::{lua_state::LuaState, t_string::tstring};

// luaconf.h:96
pub const LUA_BUFFERSIZE: usize = 512;

/// `luaL_Buffer` 的字符串累加缓冲。生命周期不变量（由 `luaL_buffinit`/`luaL_addlstring`/
/// `luaL_pushresult` 一族维护，见 functions/lua_l_buffinit.rs 等）：
/// - 初始化后 `p..end` 恒为同一分配区内的可写游标：未溢出时指向 `buffer` 内联区，
///   溢出后经 `extendstrbuf` 换入 `storage` 持有的 GC 字符串区；`p == end` 表示写满；
/// - `l`/`storage` 在 init 前为 null（cpp 同构），使用期内非空；`storage` 仅在已换入
///   GC 缓冲时指向该 TString，属借用（所有权在栈/串表），不入 GC 图句柄改造范围。
///
/// `#[repr(C)]` 是对外契约：C 宿主可自带 `luaL_Buffer` 内存经 `ulua-capi` 传入，布局须与 C 头一致。
#[repr(C)]
#[derive(Debug)]
pub struct LuaLStrbuf {
  /// 当前写游标（`buffer` 或 `storage` 区内）；init 前为 null。
  pub p: *mut c_char,
  /// 当前缓冲区的写终点（开区间界）；init 前为 null。
  pub end: *mut c_char,
  pub l: *mut LuaState,
  pub storage: *mut tstring,
  /// 内联小缓冲；cpp `alignas(LUAI_MAXALIGN)`，此处由 repr(C) 保证紧随其前字段布局。
  pub buffer: [c_char; LUA_BUFFERSIZE],
}

impl LuaLStrbuf {
  /// 空缓冲：字段由 `luaL_buffinit` / `luaL_buffinitsize` 填充。
  pub fn new() -> Self {
    Self {
      p: null_mut(),
      end: null_mut(),
      l: null_mut(),
      storage: null_mut(),
      buffer: [0; LUA_BUFFERSIZE],
    }
  }
}

// `new` + `Default` 并存是 clippy(new_without_default) 的要求，非死代码。
impl Default for LuaLStrbuf {
  fn default() -> Self {
    Self::new()
  }
}
