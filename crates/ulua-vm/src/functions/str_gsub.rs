//! Node: `cxx:Function:Luau.VM:VM/src/lstrlib.cpp:831:str_gsub`
//!
//! `string.gsub` — global substitution. Repeatedly match the pattern against the
//! source (up to `max_s` times), append each replacement via `add_value` and the
//! intervening literal text, then push the result string and the substitution
//! count. Honors a leading `^` anchor (single attempt).

use core::{
  ffi::{c_char, c_int},
  mem::zeroed,
  ptr::null_mut,
};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    add_value::add_value, lua_l_addchar::lua_l_addchar, lua_l_addlstring::lua_l_addlstring,
    lua_l_buffinit::lua_l_buffinit, lua_l_checklstring::lua_l_checklstring,
    lua_l_optinteger::lua_l_optinteger, lua_l_pushresult::lua_l_pushresult,
    lua_pushinteger::lua_pushinteger, lua_type::lua_type, r#match::match_item,
    prepstate::prepstate, reprepstate::reprepstate,
  },
  macros::lua_l_argexpected::luaL_argexpected,
  records::{
    lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
    match_state::MatchState,
  },
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn str_gsub(l: *mut lua_State) -> c_int {
  unsafe {
    let mut srcl: usize = 0;
    let mut lp: usize = 0;
    let mut src = lua_l_checklstring(l, 1, &mut srcl);
    let mut p = lua_l_checklstring(l, 2, &mut lp);
    let tr = lua_type(l, 3);
    let max_s = lua_l_optinteger(l, 4, srcl as c_int + 1);
    let anchor = *p == b'^' as c_char;
    let mut n: i32 = 0;

    let mut ms: MatchState = zeroed();
    let mut b: LuaLStrbuf = LuaLStrbuf {
      p: null_mut(),
      end: null_mut(),
      l: null_mut(),
      storage: null_mut(),
      buffer: [0; LUA_BUFFERSIZE],
    };

    luaL_argexpected!(
      l,
      tr == LuaType::Number as c_int
        || tr == LuaType::String as c_int
        || tr == LuaType::Function as c_int
        || tr == LuaType::Table as c_int,
      3,
      "string/function/table"
    );

    lua_l_buffinit(l, &mut b);

    if anchor {
      p = p.add(1);
      lp -= 1; // skip anchor character
    }

    prepstate(&mut ms, l, src, srcl, p, lp);

    while n < max_s {
      reprepstate(&mut ms);
      let e = match_item(&mut ms, src, p);
      if !e.is_null() {
        n += 1;
        add_value(&mut ms, &mut b, src, e, tr);
      }

      if !e.is_null() && e > src {
        // non empty match?
        src = e; // skip it
      } else if src < ms.src_end {
        lua_l_addchar(&mut b, *src);
        src = src.add(1);
      } else {
        break;
      }

      if anchor {
        break;
      }
    }

    lua_l_addlstring(&mut b, src, ms.src_end.offset_from(src) as usize);
    lua_l_pushresult(&mut b);
    lua_pushinteger(l, n); // number of substitutions
    2
  }
}
