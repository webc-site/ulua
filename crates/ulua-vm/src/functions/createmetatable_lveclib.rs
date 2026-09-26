use core::ptr::null;

use crate::{
  functions::{
    lua_createtable::lua_createtable,
    lua_pushvalue::lua_pushvalue,
    lua_pushvector_lapi::{
      lua_pushvector_lua_state_f32_f32_f32, lua_pushvector_lua_state_f32_f32_f32_f32,
    },
    lua_setfield::lua_setfield,
    lua_setmetatable::lua_setmetatable,
    lua_setreadonly::lua_setreadonly,
    vector_index::vector_index,
  },
  macros::{
    lua_pop::lua_pop, lua_pushcfunction::LUA_PUSHCFUNCTION, lua_vector_size::LUA_VECTOR_SIZE,
    tm_index::TM_INDEX,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且栈留有约 2 个空槽容纳建表/推入的向量与元表序列；本函数
/// `lua_createtable`/`setmetatable`/`setreadonly` 可分配并可抛错，须在受保护帧内调用。
/// cpp `lveclib.cpp:316`（vector 库 `createmetatable`）。
pub(crate) unsafe fn createmetatable(l: *mut LuaState) {
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

    lua_setfield(l, -2, TM_INDEX.as_ptr().cast());

    lua_setreadonly(l, -1, 1); // true is 1 in C API

    lua_pop(l, 1); // pop the metatable
  }
}
