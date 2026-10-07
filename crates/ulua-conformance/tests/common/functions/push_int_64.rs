use core::mem::size_of;

use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{
  k_int_64_tag::K_INT_64_TAG,
  safe_api::{newuserdatatagged, state_mut},
};
pub(crate) fn push_int_64(l: *mut LuaState, value: i64) {
  let p = newuserdatatagged(l, size_of::<i64>(), K_INT_64_TAG);

  state_mut(l).get_metatable_by_str("int64");
  state_mut(l).set_metatable(-2);

  // `p` 为刚分配的 K_INT_64_TAG userdata 数据区，容量即 size_of::<i64>()。
  // Safety: 尺寸/对齐由上一行 size_of 实参配对保证，仅本行写一次。
  unsafe { *(p as *mut i64) = value };
}
