use core::{ffi::c_int, ptr::null};

use ulua_vm::{
  functions::{
    lua_l_newmetatable::lua_l_newmetatable,
    lua_pushcclosurek::lua_pushcclosurek,
    lua_pushstring::lua_pushstring,
    lua_pushvector_lapi::{
      lua_pushvector_lua_state_f32_f32_f32, lua_pushvector_lua_state_f32_f32_f32_f32,
    },
    lua_setmetatable::lua_setmetatable,
    lua_setreadonly::lua_setreadonly,
    lua_settable::lua_settable,
  },
  macros::{lua_pop::lua_pop, lua_vector_size::LUA_VECTOR_SIZE},
  records::lua_state::LuaState,
};

use crate::common::functions::{
  cstr::cstr, lua_vector_index::lua_vector_index, lua_vector_namecall::lua_vector_namecall,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn setup_vector_helpers(l: *mut LuaState) {
  const METHODS: [(&[u8], unsafe extern "C-unwind" fn(*mut LuaState) -> c_int); 2] = [
    (b"__index\0", lua_vector_index),
    (b"__namecall\0", lua_vector_namecall),
  ];

  // Safety: `l` 为本用例存活的 LuaState；按构建期 LUA_VECTOR_SIZE 压入零向量
  // （两条分支各自写全部分量，无悬垂指针）。
  unsafe {
    if LUA_VECTOR_SIZE == 4 {
      lua_pushvector_lua_state_f32_f32_f32_f32(l, 0.0, 0.0, 0.0, 0.0);
    } else {
      lua_pushvector_lua_state_f32_f32_f32(l, 0.0, 0.0, 0.0);
    }
  }

  // Safety: `l` 存活；建/取 vector metatable，置于栈顶。
  unsafe { lua_l_newmetatable(l, cstr(b"vector\0")) };

  // Safety: `l` 存活且栈顶为 metatable；循环内 push string + pushcclosurek 后
  // lua_settable(-3) 消费两者，栈形不变，`METHODS` 的名字 NUL 结尾、函数为
  // `extern "C-unwind"` 桩。
  unsafe {
    for &(name, func) in &METHODS {
      lua_pushstring(l, name.as_ptr().cast());
      lua_pushcclosurek(l, Some(func), null(), 0, None);
      lua_settable(l, -3);
    }
  }

  // Safety: `l` 存活且栈顶仍是 metatable、其下为向量对象：置只读、给向量绑元表后弹掉元表。
  unsafe {
    lua_setreadonly(l, -1, 1);
    lua_setmetatable(l, -2);
    lua_pop(l, 1);
  }
}
