use core::mem::size_of;

use ulua_vm::{
  functions::{lua_newuserdatatagged::lua_newuserdatatagged, lua_setmetatable::lua_setmetatable},
  macros::lua_l_getmetatable::lua_l_getmetatable,
  records::lua_state::lua_State,
};

use crate::common::functions::k_int_64_tag::K_INT_64_TAG;
pub(crate) fn push_int_64(l: *mut lua_State, value: i64) {
  unsafe {
    let p = lua_newuserdatatagged(l, size_of::<i64>(), K_INT_64_TAG);

    lua_l_getmetatable(l, c"int64".as_ptr());
    lua_setmetatable(l, -2);

    *(p as *mut i64) = value;
  }
}
