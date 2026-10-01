use ulua_vm::records::lua_state::LuaState;

use crate::common::{
  functions::{lua_vec_2_push::lua_vec_2_push},
  records::vec_2_conformance_ir_hooks::Vec2,
};

pub(crate) fn lua_vec_2_clone(l: *mut LuaState, self_ptr: *mut Vec2) -> i32 {
  // `lua_vec_2_push` 是 safe 门面：新建 Vec2 userdata 并返回其数据指针。
  let r_ptr = lua_vec_2_push(l);

  // Safety: `self_ptr` 指向刚校验的 Vec2 数据、`r_ptr` 为刚新建的数据区，各触碰一次。
  unsafe {
    (*r_ptr).x = (*self_ptr).x;
    (*r_ptr).y = (*self_ptr).y;
  }
  1
}
