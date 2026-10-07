use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

use crate::common::{
  functions::safe_api::{l_typeerror, touserdatatagged},
  records::{userdata_tags::K_TAG_VEC2, vec_2_conformance_ir_hooks::Vec2},
};
/// # Safety
///
pub fn lua_vec_2_get(l: *mut LuaState, idx: i32) -> *mut Vec2 {
  let a = touserdatatagged(l, idx, K_TAG_VEC2 as c_int) as *mut Vec2;

  if !a.is_null() {
    return a;
  }

  // tag 不符按 cpp 抛「不是 vec2」的类型错误，该调用不返回。
  l_typeerror(l, idx, "vec2")
}
