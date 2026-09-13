use core::ffi::c_int;

use crate::{
  functions::{
    lua_pushinteger::lua_pushinteger, lua_replace::lua_replace, lua_tolstring::lua_tolstring,
    r#match::match_item, prepstate::prepstate, push_captures::push_captures,
    reprepstate::reprepstate,
  },
  macros::{lua_tointeger::lua_tointeger, lua_upvalueindex::lua_upvalueindex},
  records::match_state::MatchState,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn gmatch_aux(l: *mut lua_State) -> c_int {
  unsafe {
    let mut ms = MatchState::default();
    let mut ls: usize = 0;
    let mut lp: usize = 0;
    let s = lua_tolstring(l, lua_upvalueindex(1), &mut ls);
    let p = lua_tolstring(l, lua_upvalueindex(2), &mut lp);

    prepstate(&mut ms, l, s, ls, p, lp);

    let mut src = s.add(lua_tointeger!(l, lua_upvalueindex(3)) as usize);
    while src <= ms.src_end {
      reprepstate(&mut ms);
      let e = match_item(&mut ms, src, p);
      if !e.is_null() {
        let mut newstart = e.offset_from(s) as c_int;
        if e == src {
          newstart += 1;
        }
        lua_pushinteger(l, newstart);
        lua_replace(l, lua_upvalueindex(3));
        return push_captures(&mut ms, src, e);
      }
      src = src.add(1);
    }

    0
  }
}
