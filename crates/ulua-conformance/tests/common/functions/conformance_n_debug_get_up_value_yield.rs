use core::mem::zeroed;

use ulua_vm::{
  functions::{
    lua_checkstack::lua_checkstack, lua_getinfo::lua_getinfo, lua_getupvalue::lua_getupvalue,
  },
  macros::{lua_minstack::LUA_MINSTACK, lua_pop::lua_pop, lua_tointeger::lua_tointeger},
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

use crate::common::functions::{cstr::cstr, cstr_text::cstr_raw};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_n_debug_get_up_value_yield(l: *mut LuaState) -> bool {
  // Safety: `l` 为本用例存活的 LuaState；确保 Lua 帧可压栈（LUA_MINSTACK 个槽位）。
  unsafe { lua_checkstack(l, LUA_MINSTACK) };

  // Safety: `ar` 由 `zeroed()` 置为全零 LuaDebug（所有字段都可全零表示），
  // 随后 lua_getinfo 按掩码 'f' 填充函数原型槽。
  let mut ar: LuaDebug = unsafe { zeroed() };
  // Safety: `l` 存活、1 为调用方帧的合法层级；`b"f\0"` 为 NUL 结尾掩码，
  // `&mut ar` 指向上一步已初始化的记录。
  assert_ne!(0, unsafe { lua_getinfo(l, 1, cstr(b"f\0"), &mut ar) });

  // Safety: `l` 存活且栈顶为刚取到的函数原型；取第 1 个上值的名字指针（断言非空后读）。
  let upvalue = unsafe { lua_getupvalue(l, -1, 1) };
  assert!(!upvalue.is_null());
  // Safety: 上一断言保证 `upvalue` 非空且为 NUL 结尾串。
  assert_eq!(unsafe { cstr_raw(upvalue) }, b"");
  // Safety: `l` 存活、栈顶仍为该上值的值槽（`lua_tointeger!` 不可转换时返回 0）。
  assert_eq!(unsafe { lua_tointeger!(l, -1) }, 5);
  // Safety: `l` 存活；弹掉原型与上值两个槽，恢复进入本续体时的栈形。
  unsafe { lua_pop(l, 2) };

  false
}
