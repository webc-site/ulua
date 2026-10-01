use core::ptr::null;

use crate::{
  functions::{
    lua_createtable::lua_createtable,
    lua_pushvector_lapi::{
      lua_pushvector_lua_state_f32_f32_f32, lua_pushvector_lua_state_f32_f32_f32_f32,
    },
    vector_index::vector_index_arm,
  },
  macros::lua_vector_size::LUA_VECTOR_SIZE,
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

    (*l).push_value(-2);

    (*l).set_metatable(-2); // set vector metatable
    (*l).pop(1); // pop dummy vector

    (*l).push_c_function(Some(vector_index_arm), null());

    (*l).set_field_str(-2, "__index");

    (*l).set_readonly(-1, true); // true is 1 in C API

    (*l).pop(1); // pop the metatable
  }
}
