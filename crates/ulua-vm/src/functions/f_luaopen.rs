//! Node: `cxx:Function:Luau.VM:VM/src/lstate.cpp:59:f_luaopen`
//! Source: `VM/src/lstate.cpp:59-70` (hand-ported)

use core::ffi::c_void;

use crate::{
  functions::{
    lua_h_new::lua_h_new, lua_s_resize::luaS_resize, lua_t_init::lua_t_init, stack_init::stack_init,
  },
  macros::{
    lua_minstrtabsize::LUA_MINSTRTABSIZE, lua_s_fix::luaS_fix, lua_s_newliteral::lua_s_newliteral,
    registry::registry, sethvalue::sethvalue,
  },
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// open parts that may cause memory-allocation errors
pub(crate) unsafe extern "C-unwind" fn f_luaopen(l: *mut lua_State, _ud: *mut c_void) {
  unsafe {
    let g = (*l).global;
    stack_init(l, l); // init stack
    (*l).gt = lua_h_new(l, 0, 2); // table of globals
    sethvalue!(
      l,
      registry!(l) as *const TValue as *mut TValue,
      lua_h_new(l, 0, 2)
    ); // registry
    luaS_resize(l, LUA_MINSTRTABSIZE); // initial size of string table
    lua_t_init(l);
    luaS_fix!(lua_s_newliteral(l, c"not enough memory".as_ptr())); // LUA_MEMERRMSG // pin to make sure we can always throw this error
    luaS_fix!(lua_s_newliteral(l, c"error in error handling".as_ptr())); // LUA_ERRERRMSG // pin to make sure we can always throw this error
    (*g).gc_threshold = 4 * (*g).totalbytes;
  }
}
