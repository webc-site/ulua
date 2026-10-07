use core::ffi::c_int;

use ulua_vm::{macros::lua_vector_size::LUA_VECTOR_SIZE, records::lua_state::LuaState};

use crate::common::functions::{
  lua_vector_index::lua_vector_index,
  lua_vector_namecall::lua_vector_namecall,
  safe_api::{pushcclosurek, pushvector3, pushvector4, settable, state_mut},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub fn setup_vector_helpers(l: *mut LuaState) {
  const METHODS: [(&[u8], unsafe extern "C-unwind" fn(*mut LuaState) -> c_int); 2] = [
    (b"__index", lua_vector_index),
    (b"__namecall", lua_vector_namecall),
  ];

  // 按构建期 LUA_VECTOR_SIZE 压入零向量（两条分支各自写全部分量）。
  if LUA_VECTOR_SIZE == 4 {
    pushvector4(l, 0.0, 0.0, 0.0, 0.0);
  } else {
    pushvector3(l, 0.0, 0.0, 0.0);
  }

  // 建/取 vector metatable，置于栈顶。
  state_mut(l).new_metatable_by_str("vector");

  // 循环内 push string + pushcclosurek 后 settable(-3) 消费两者，栈形不变，
  // `METHODS` 的名字为 NUL 结尾静态串、函数为 `extern "C-unwind"` 桩。
  for &(name, func) in &METHODS {
    state_mut(l).push_bytes(name);
    pushcclosurek(l, Some(func), None, 0, None);
    settable(l, -3);
  }

  // 栈顶仍是 metatable、其下为向量对象：置只读、给向量绑元表后弹掉元表。
  state_mut(l).set_readonly(-1, true);
  state_mut(l).set_metatable(-2);
  state_mut(l).pop(1);
}
