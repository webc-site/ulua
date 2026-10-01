use core::{ffi::c_int, mem::size_of};

use ulua_vm::records::lua_state::LuaState;

use crate::common::{
  functions::safe_api::{getuserdatametatable, newuserdatatagged, state_mut},
  records::{userdata_tags::K_TAG_VEC2, vec_2_conformance_ir_hooks::Vec2},
};

pub(crate) fn lua_vec_2_push(l: *mut LuaState) -> *mut Vec2 {
  let data = newuserdatatagged(l, size_of::<Vec2>(), K_TAG_VEC2 as c_int) as *mut Vec2;

  getuserdatametatable(l, K_TAG_VEC2 as c_int);

  state_mut(l).set_metatable(-2);

  data
}
