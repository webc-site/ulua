use core::ffi::c_char;

use crate::{
  functions::lua_pushcclosurek::lua_pushcclosurek,
  type_aliases::{lua_c_function::LuaCFunction, lua_state::lua_State},
};

pub const LUA_PUSHCFUNCTION: unsafe fn(*mut lua_State, LuaCFunction, *const c_char) =
  |l, f, debugname| unsafe {
    lua_pushcclosurek(l, f, debugname, 0, None);
  };
