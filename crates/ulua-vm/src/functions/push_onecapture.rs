use core::ffi::{c_char, c_int};

use crate::{
  functions::{lua_pushinteger::lua_pushinteger, lua_pushlstring::lua_pushlstring},
  macros::{cap_position::CAP_POSITION, cap_unfinished::CAP_UNFINISHED, lua_l_error::luaL_error},
  records::match_state::MatchState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn push_onecapture(
  ms: *mut MatchState,
  i: c_int,
  s: *const c_char,
  e: *const c_char,
) {
  unsafe {
    if i >= (*ms).level {
      if i == 0 {
        // lua_pushlstring(ms->l, s, e - s);
        lua_pushlstring((*ms).l, s, e.offset_from(s) as usize);
      } else {
        luaL_error!((*ms).l, "invalid capture index");
      }
    } else {
      let l = (*ms).capture[i as usize].len;
      if l == CAP_UNFINISHED as isize {
        luaL_error!((*ms).l, "unfinished capture");
      } else if l == CAP_POSITION as isize {
        // lua_pushinteger(ms->l, (int)(ms->capture[i].init - ms->src_init) + 1);
        let pos = (*ms).capture[i as usize].init.offset_from((*ms).src_init) as c_int;
        lua_pushinteger((*ms).l, pos + 1);
      } else {
        // lua_pushlstring(ms->l, ms->capture[i].init, l);
        lua_pushlstring((*ms).l, (*ms).capture[i as usize].init, l as usize);
      }
    }
  }
}
