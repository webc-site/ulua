use ulua_vm::records::lua_state::LuaState;

use crate::common::{
  functions::{lua_vec_2_get::lua_vec_2_get, safe_api::state_mut},
  records::vec_2_conformance_ir_hooks::Vec2,
};

pub(crate) fn lua_vec_2_dot(l: *mut LuaState, self_ptr: *mut Vec2) -> i32 {
  let b_ptr = lua_vec_2_get(l, 2);

  // Safety: 两个指针均指向刚校验的 Vec2 userdata 数据区，仅本段读一次。
  let result = unsafe {
    ((*self_ptr).x as f64 * (*b_ptr).x as f64) + ((*self_ptr).y as f64 * (*b_ptr).y as f64)
  };

  state_mut(l).push_number(result);
  1
}
