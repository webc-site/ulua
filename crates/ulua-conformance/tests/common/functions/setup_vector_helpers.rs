use core::{
  ffi::{CStr, c_int},
  ptr::null,
};

use ulua_vm::{
  functions::{
    lua_l_newmetatable::lua_l_newmetatable, lua_pushcclosurek::lua_pushcclosurek,
    lua_pushstring::lua_pushstring, lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32_f32,
    lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32,
    lua_setmetatable::lua_setmetatable, lua_setreadonly::lua_setreadonly,
    lua_settable::lua_settable,
  },
  macros::{lua_pop::lua_pop, lua_vector_size::LUA_VECTOR_SIZE},
  records::lua_state::lua_State,
};

use crate::common::functions::{
  lua_vector_index::lua_vector_index, lua_vector_namecall::lua_vector_namecall,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn setup_vector_helpers(l: *mut lua_State) {
  unsafe {
    if LUA_VECTOR_SIZE == 4 {
      lua_pushvector_lua_state_f32_f32_f32_f32(l, 0.0, 0.0, 0.0, 0.0);
    } else {
      lua_pushvector_lua_state_f32_f32_f32(l, 0.0, 0.0, 0.0);
    }

    lua_l_newmetatable(l, c"vector".as_ptr());

    const METHODS: [(&CStr, unsafe extern "C-unwind" fn(*mut lua_State) -> c_int); 2] = [
      (c"__index", lua_vector_index),
      (c"__namecall", lua_vector_namecall),
    ];

    for &(name, func) in &METHODS {
      lua_pushstring(l, name.as_ptr());
      lua_pushcclosurek(l, Some(func), null(), 0, None);
      lua_settable(l, -3);
    }

    lua_setreadonly(l, -1, 1);
    lua_setmetatable(l, -2);
    lua_pop(l, 1);
  }
}
