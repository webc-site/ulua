//! Source: `VM/src/lstrlib.cpp:798`
//!
//! `string.gsub` replacement dispatch for one match: a function replacement is
//! called with the captures, a table replacement is indexed by the first
//! capture, and a string/number replacement goes through `add_s`. A falsy or
//! non-string result falls back to the original matched text.

use ulua_common::functions::c_str::cstr_cow;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    add_s::add_s, lua_gettable::lua_gettable, lua_isstring::lua_isstring,
    lua_l_addvalue::lua_l_addvalue, lua_l_typename::lua_l_typename,
    lua_pushlstring::lua_pushlstring_bytes, push_captures::push_captures,
    push_onecapture::push_onecapture,
  },
  macros::lua_l_error::luaL_error,
  records::{lua_l_strbuf::LuaLStrbuf, match_state::MatchState},
};

/// cpp `lstrlib.cpp add_value`：一次匹配的替换分派（函数/表/串数值）。
/// `s`/`e` 为整窗匹配的源偏移对。
///
/// # Safety
///
/// `ms` 及其 src/pattern 与捕获槽必须来自 `prepstate` 建立、仍处本次匹配
/// 调用中的 `MatchState`；`s <= e <= ms.src.len()`；栈存活且第 2/3 槽按索引可读。
pub(crate) unsafe fn add_value(
  ms: &mut MatchState,
  b: &mut LuaLStrbuf,
  s: usize,
  e: usize,
  tr: LuaType,
) {
  // SAFETY: 契约保证 `l` 存活且栈顶第 2 槽为可读串/数值 TValue，块内取值仅在源串数据界内拼接
  unsafe {
    let l = ms.l;
    if tr == LuaType::Function {
      (*l).push_value(3);
      let n = push_captures(ms, Some(s), Some(e));
      (*l).call(n, 1);
    } else if tr == LuaType::Table {
      push_onecapture(ms, 0, Some(s), Some(e));
      lua_gettable(&mut *l, 3);
    } else {
      // LUA_TNUMBER or LUA_TSTRING
      add_s(ms, b, s, e);
      return;
    }

    if !(*l).to_boolean(-1) {
      // nil or false?
      (*l).pop(1);
      // cpp: lua_pushlstring(L, src, e - src); keep original text
      // —— Rust 侧直投切片 ref 核心，不再经 C-API ptr+len 形重建指针
      // SAFETY: src_slice 按契约（s <= e <= src.len()）返回界内切片；
      // lua_pushlstring_bytes 仅界内拷入堆串、不留存借用
      let keep = ms.src_slice(s, e - s);
      lua_pushlstring_bytes(&mut *l, keep); // keep original text
    } else if lua_isstring(&*l, -1) == 0 {
      let tn = cstr_cow(lua_l_typename(&*l, -1));
      luaL_error!(l, "invalid replacement value (a {})", tn);
    }
    lua_l_addvalue(b); // add result to accumulator
  }
}
