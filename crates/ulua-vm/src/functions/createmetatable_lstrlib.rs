use core::ffi::c_void;

use crate::{
  functions::{
    lua_createtable::lua_createtable, lua_pushvalue::lua_pushvalue, lua_setfield::lua_setfield,
    lua_setmetatable::lua_setmetatable,
  },
  macros::{lua_pop::lua_pop, lua_pushliteral::LUA_PUSHLITERAL},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn createmetatable_mut(l: *mut lua_State) {
  unsafe {
    lua_createtable(l, 0, 1); // create metatable for strings

    LUA_PUSHLITERAL(l as *mut c_void, c"".as_ptr()); // dummy string

    lua_pushvalue(l, -2);

    lua_setmetatable(l, -2); // set string metatable

    lua_pop(l, 1); // pop dummy string

    lua_pushvalue(l, -2); // string library...

    lua_setfield(l, -2, c"__index".as_ptr()); // ...is the __index metamethod

    lua_pop(l, 1); // pop metatable
  }
}
