use core::ptr::null;

use crate::{
  functions::{
    lua_createtable::lua_createtable, lua_pushvalue::lua_pushvalue,
    lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32_f32,
    lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32, lua_setfield::lua_setfield,
    lua_setmetatable::lua_setmetatable, lua_setreadonly::lua_setreadonly,
    vector_index::vector_index,
  },
  macros::{
    lua_pop::lua_pop, lua_pushcfunction::LUA_PUSHCFUNCTION, lua_vector_size::LUA_VECTOR_SIZE,
  },
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn createmetatable(l: *mut lua_State) {
  unsafe {
    lua_createtable(l, 0, 1); // create metatable for vectors

    // push dummy vector
    if LUA_VECTOR_SIZE == 4 {
      lua_pushvector_lua_state_f32_f32_f32_f32(l, 0.0, 0.0, 0.0, 0.0);
    } else {
      lua_pushvector_lua_state_f32_f32_f32(l, 0.0, 0.0, 0.0);
    }

    lua_pushvalue(l, -2);

    lua_setmetatable(l, -2); // set vector metatable
    lua_pop(l, 1); // pop dummy vector

    LUA_PUSHCFUNCTION(l, Some(vector_index), null());

    lua_setfield(l, -2, c"__index".as_ptr());

    lua_setreadonly(l, -1, 1); // true is 1 in C API

    lua_pop(l, 1); // pop the metatable
  }
}
