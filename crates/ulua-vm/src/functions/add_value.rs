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
/// w6e 诚实降级：形参已全为引用/整数/Copy 句柄裸形（`ms.l` 本轮保留裸形，见
/// `MatchState` 注记），真实裸操作（帧方法面解引用、`lua_gettable`/`lua_isstring`/
/// `cstr_cow` 引用重建、`lua_l_addvalue` 栈消费）落逐句窄 `unsafe` 块。
///
/// 调用序契约（正确性，非内存安全）：`ms` 及其 src/pattern 与捕获槽必须来自
/// `prepstate` 建立、仍处本次匹配调用中的 `MatchState`；`s <= e <= ms.src.len()`；
/// 栈存活且第 2/3 槽按索引可读。
pub(crate) fn add_value(ms: &mut MatchState, b: &mut LuaLStrbuf, s: usize, e: usize, tr: LuaType) {
  let l = ms.l;
  if tr == LuaType::Function {
    // SAFETY: `l` 为 prepstate 接线、本次匹配调用内存活的帧句柄（调用序契约），
    // 块内仅经句柄走安全方法面（push_value/call 自身为 safe fn）
    unsafe { (*l).push_value(3) };
    let n = push_captures(ms, Some(s), Some(e));
    // SAFETY: 同上帧句柄；`call` 回跑 Lua 函数（可抛错/触发 GC），按库约定结果
    // 留栈顶返回，不跨调用持旧栈槽指针
    unsafe { (*l).call(n, 1) };
  } else if tr == LuaType::Table {
    push_onecapture(ms, 0, Some(s), Some(e));
    // SAFETY: `&mut *l` 就地重建借用窗（句止当句）；`lua_gettable` 已收 safe 引用形，
    // 3 号槽为替换表（gsub 契约）
    unsafe { lua_gettable(&mut *l, 3) };
  } else {
    // LUA_TNUMBER or LUA_TSTRING
    add_s(ms, b, s, e);
    return;
  }

  // SAFETY: `l` 存活帧句柄（调用序契约）；`to_boolean` 走 safe 方法面读栈顶槽
  if !unsafe { (*l).to_boolean(-1) } {
    // nil or false?
    // SAFETY: 存活帧句柄上的安全方法面（pop 只动 top，不解引用已死槽）
    unsafe { (*l).pop(1) };
    // cpp: lua_pushlstring(L, src, e - src); keep original text
    // —— Rust 侧直投切片 ref 核心，不再经 C-API ptr+len 形重建指针
    let keep = ms.src_slice(s, e - s);
    // SAFETY: src_slice 按契约（s <= e <= src.len()）返回界内切片；存活帧句柄；
    // lua_pushlstring_bytes 仅界内拷入堆串、不留存借用
    unsafe { lua_pushlstring_bytes(&mut *l, keep) }; // keep original text
  }
  // SAFETY: 存活帧句柄；`lua_isstring` 为 safe 谓词，纯读栈顶槽
  else if unsafe { lua_isstring(&*l, -1) } == 0 {
    // SAFETY: 存活帧句柄；`cstr_cow` 契约——`lua_l_typename` 返回 NUL 终止可读静态
    // 类型名串；`lua_l_error_l` raise 后发散
    unsafe {
      let tn = cstr_cow(lua_l_typename(&*l, -1));
      luaL_error!(l, "invalid replacement value (a {})", tn);
    }
  }
  // SAFETY: `b` 为本次 gsub 登记的累加器、栈顶为替换结果串（上方两分支之一建立）；
  // lua_l_addvalue 弹出栈顶并追加
  unsafe { lua_l_addvalue(b) }; // add result to accumulator
}
