use core::{
  ffi::{c_char, c_int},
  mem::transmute,
};

use crate::{
  functions::{
    lua_l_error_l::lua_l_error_l, lua_pushinteger::lua_pushinteger,
    lua_pushlstring::lua_pushlstring,
  },
  macros::{cap_position::CAP_POSITION, cap_unfinished::CAP_UNFINISHED},
  records::{lua_state::LuaState, match_state::MatchState},
};

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
        let len = (e as usize).wrapping_sub(s as usize);
        // The dependency card shows lua_pushlstring() with no args in the stub,
        // but the contract requires calling with real arguments.
        let pushlstring_ptr = lua_pushlstring as *const ();
        let pushlstring_fn: unsafe fn(*mut LuaState, *const c_char, usize) =
          transmute(pushlstring_ptr);
        pushlstring_fn((*ms).l, s, len);
      } else {
        // The luaL_error macro expansion calls lua_l_error_l.
        // Per contract: "Pass &str to a callee even if its current stub signature still shows *const i8".
        // However, the compiler error shows the current stub for lua_l_error_l expects *const c_char.
        // We cast the &str to a pointer to satisfy the current stub while it is being updated.
        let fmt = "invalid capture index";
        lua_l_error_l(
          (*ms).l,
          fmt.as_ptr() as *const c_char,
          core::format_args!("{}", fmt),
        );
      }
    } else {
      let l = (*ms).capture[i as usize].len;
      if l == CAP_UNFINISHED as isize {
        let fmt = "unfinished capture";
        lua_l_error_l(
          (*ms).l,
          fmt.as_ptr() as *const c_char,
          core::format_args!("{}", fmt),
        );
      } else if l == CAP_POSITION as isize {
        // lua_pushinteger(ms->l, (int)(ms->capture[i].init - ms->src_init) + 1);
        let pos =
          ((*ms).capture[i as usize].init as usize).wrapping_sub((*ms).src_init as usize) as c_int;
        lua_pushinteger((*ms).l, pos + 1);
      } else {
        // lua_pushlstring(ms->l, ms->capture[i].init, l);
        let pushlstring_ptr = lua_pushlstring as *const ();
        let pushlstring_fn: unsafe fn(*mut LuaState, *const c_char, usize) =
          transmute(pushlstring_ptr);
        pushlstring_fn((*ms).l, (*ms).capture[i as usize].init, l as usize);
      }
    }
  }
}
