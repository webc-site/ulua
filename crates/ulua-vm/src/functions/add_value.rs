//! Node: `cxx:Function:Luau.VM:VM/src/lstrlib.cpp:796:add_value`
//!
//! `string.gsub` replacement dispatch for one match: a function replacement is
//! called with the captures, a table replacement is indexed by the first
//! capture, and a string/number replacement goes through `add_s`. A falsy or
//! non-string result falls back to the original matched text.

use core::ffi::{CStr, c_char};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    add_s::add_s, lua_call::lua_call, lua_gettable::lua_gettable, lua_isstring::lua_isstring,
    lua_l_addvalue::lua_l_addvalue, lua_l_typename::lua_l_typename,
    lua_pushlstring::lua_pushlstring, lua_pushvalue::lua_pushvalue, lua_toboolean::lua_toboolean,
    push_captures::push_captures, push_onecapture::push_onecapture,
  },
  macros::{lua_l_error::luaL_error, lua_pop::lua_pop},
  records::{lua_l_strbuf::LuaLStrbuf, match_state::MatchState},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn add_value(
  ms: *mut MatchState,
  b: *mut LuaLStrbuf,
  s: *const c_char,
  e: *const c_char,
  tr: i32,
) {
  unsafe {
    let l = (*ms).l;
    if tr == LuaType::Function as i32 {
      lua_pushvalue(l, 3);
      let n = push_captures(ms, s, e);
      lua_call(l, n, 1);
    } else if tr == LuaType::Table as i32 {
      push_onecapture(ms, 0, s, e);
      lua_gettable(l, 3);
    } else {
      // LUA_TNUMBER or LUA_TSTRING
      add_s(ms, b, s, e);
      return;
    }

    if lua_toboolean(l, -1) == 0 {
      // nil or false?
      lua_pop(l, 1);
      lua_pushlstring(l, s, e.offset_from(s) as usize); // keep original text
    } else if lua_isstring(l, -1) == 0 {
      let tn = CStr::from_ptr(lua_l_typename(l, -1)).to_string_lossy();
      luaL_error!(l, "invalid replacement value (a {})", tn);
    }
    lua_l_addvalue(b); // add result to accumulator
  }
}
