use ulua_vm::{macros::lua_minstack::LUA_MINSTACK, records::lua_state::LuaState};

use crate::common::functions::{
  cstr_text::cstr_raw,
  safe_api::{checkstack, getinfo, getupvalue, state_mut, zero_debug},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_n_debug_get_up_value_yield(l: *mut LuaState) -> bool {
  // 确保 Lua 帧可压栈（LUA_MINSTACK 个槽位）。
  checkstack(l, LUA_MINSTACK);

  // `zero_debug` 交出全零 LuaDebug，getinfo 按掩码 'f' 填充函数原型槽。
  let mut ar = zero_debug();
  assert_ne!(0, getinfo(l, 1, b"f\0", &mut ar));

  // 栈顶为刚取到的函数原型；取第 1 个上值的名字指针（断言非空后读）。
  let upvalue = getupvalue(l, -1, 1);
  assert!(!upvalue.is_null());
  // Safety: 上一断言保证 `upvalue` 非空且为 NUL 结尾串。
  assert_eq!(unsafe { cstr_raw(upvalue) }, b"");
  // 栈顶仍为该上值的值槽（`lua_tointeger!` 不可转换时返回 0）。
  assert_eq!(state_mut(l).to_integer(-1).unwrap_or(0), 5);
  // 弹掉原型与上值两个槽，恢复进入本续体时的栈形。
  state_mut(l).pop(2);

  false
}
