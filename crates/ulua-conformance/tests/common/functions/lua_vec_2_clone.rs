use ulua_vm::records::lua_state::lua_State;

use crate::common::{
  functions::lua_vec_2_push::lua_vec_2_push, records::vec_2_conformance_ir_hooks::Vec2,
};

pub(crate) fn lua_vec_2_clone(l: *mut lua_State, self_ptr: *mut Vec2) -> i32 {
  unsafe {
    let r_ptr = lua_vec_2_push(l);

    let self_val = &*self_ptr;
    let r_val = &mut *r_ptr;

    r_val.x = self_val.x;
    r_val.y = self_val.y;
  }
  1
}
