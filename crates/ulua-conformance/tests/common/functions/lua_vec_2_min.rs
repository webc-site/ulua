use ulua_vm::records::lua_state::lua_State;

use crate::common::{
  functions::{lua_vec_2_get::lua_vec_2_get, lua_vec_2_push::lua_vec_2_push},
  records::vec_2_conformance_ir_hooks::Vec2,
};

pub(crate) fn lua_vec_2_min(l: *mut lua_State, self_ptr: *mut Vec2) -> i32 {
  unsafe {
    let b_ptr = lua_vec_2_get(l, 2);
    let data = lua_vec_2_push(l);

    let self_val = &*self_ptr;
    let b_val = &*b_ptr;

    (*data).x = if self_val.x < b_val.x {
      self_val.x
    } else {
      b_val.x
    };
    (*data).y = if self_val.y < b_val.y {
      self_val.y
    } else {
      b_val.y
    };
  }
  1
}
