use core::{ffi::c_int, mem::size_of};

use ulua_vm::{
  functions::{
    lua_getuserdatametatable::lua_getuserdatametatable,
    lua_newuserdatatagged::lua_newuserdatatagged, lua_setmetatable::lua_setmetatable,
  },
  records::lua_state::lua_State,
};

use crate::common::records::vec_2_conformance_ir_hooks::Vec2;
pub const K_TAG_VEC2: i32 = 12;

pub(crate) fn lua_vec_2_push(l: *mut lua_State) -> *mut Vec2 {
  unsafe {
    let data = lua_newuserdatatagged(l, size_of::<Vec2>(), K_TAG_VEC2 as c_int) as *mut Vec2;

    lua_getuserdatametatable(l, K_TAG_VEC2 as c_int);

    lua_setmetatable(l, -2);

    data
  }
}
