use ulua_vm::records::lua_state::LuaState;

use crate::common::{
  functions::safe_api::{pcall, state_mut},
  records::vec_2_conformance_ir_hooks::Vec2,
};

pub(crate) fn lua_vec_2_reenter(l: *mut LuaState, self_ptr: *mut Vec2) -> i32 {
  let lv = state_mut(l);
  lv.get_global_bytes(b"reenterCallback");
  assert!(lv.is_function(-1));
  pcall(l, 0, 0, 0);

  // Safety: `self_ptr` 指向刚校验的 Vec2 数据区，仅本行读一次。
  let result = unsafe { ((*self_ptr).x as f64) + ((*self_ptr).y as f64) };
  state_mut(l).push_number(result);
  1
}
