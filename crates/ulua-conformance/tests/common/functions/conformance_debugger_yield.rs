use core::{ffi::c_int, ptr::null_mut, sync::atomic::Ordering};

use ulua_vm::{
  macros::lua_minstack::LUA_MINSTACK, records::lua_state::LuaState,
};

use crate::common::{
  functions::{
    cstr_text::cstr_raw, safe_api::{checkstack, getargument, getinfo, getlocal, getupvalue, state_mut, zero_debug},
  },
  records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE,
};

/// 校验栈顶整数值并弹出（cpp `assertStackInteger` lambda）。
fn assert_stack_integer(l: *mut LuaState, value: c_int) {
  assert_eq!(state_mut(l).to_integer(-1).unwrap_or(0), value);
  state_mut(l).pop(1);
}

/// 校验 level/slot 处局部变量的名字与值（cpp `assertLocal` lambda）。
fn assert_local(l: *mut LuaState, level: c_int, slot: c_int, name: &[u8], value: c_int) {
  let local = getlocal(l, level, slot);
  assert!(!local.is_null());
  // Safety: `local` 为 VM 交回的 NUL 结尾局部变量名（上一行断言非空）。
  assert_eq!(unsafe { cstr_raw(local) }, name);
  assert_stack_integer(l, value);
}

/// breakhits==1 分支：校验两个实参、局部变量 b 与闭包 upvalue a（cpp 首命中）。
fn debugger_yield_break1(l: *mut LuaState) {
  assert_ne!(0, getargument(l, 0, 1));
  assert_stack_integer(l, 50);

  assert_ne!(0, getargument(l, 0, 2));
  assert_stack_integer(l, 42);

  assert_local(l, 0, 1, b"b", 50);

  // getinfo 的 `f` 选项只填 ar.source 一类字段且本分支不读取，
  // upvalue 校验对栈的影响以 `lua_pop(l, 2)` 收尾配平。
  let mut ar = zero_debug();
  getinfo(l, 0, b"f\0", &mut ar);

  let upvalue = getupvalue(l, -1, 1);
  assert!(!upvalue.is_null());
  // Safety: `upvalue` 为 VM 交回的 NUL 结尾上值名（上一行断言非空）。
  assert_eq!(unsafe { cstr_raw(upvalue) }, b"a");
  assert_eq!(state_mut(l).to_integer(-1).unwrap_or(0), 5);
  state_mut(l).pop(2);
}

/// breakhits==13 分支：局部变量 `a` 存在且为 nil。
fn assert_nil_local_a(l: *mut LuaState) {
  let local = getlocal(l, 0, 1);
  assert!(!local.is_null());
  // Safety: `local` 为 VM 交回的 NUL 结尾局部变量名（上一行断言非空）。
  assert_eq!(unsafe { cstr_raw(local) }, b"a");
  assert!(state_mut(l).is_nil(-1));
  state_mut(l).pop(1);
}

/// breakhits==15 分支：level 2 的 slot 1 是 `x`、slot 2 越界为空。
fn assert_level2_locals(l: *mut LuaState) {
  let x = getlocal(l, 2, 1);
  assert!(!x.is_null());
  // Safety: `x` 为 VM 交回的 NUL 结尾局部变量名（上一行断言非空）。
  assert_eq!(unsafe { cstr_raw(x) }, b"x");
  state_mut(l).pop(1);

  let a1 = getlocal(l, 2, 2);
  assert!(a1.is_null());
}

/// 若有用例登记了待恢复线程则取出并 resume（cpp debuggerHook 收尾段）。
fn resume_interrupted_thread() {
  let Some(interruptedthread) = CONFORMANCE_DEBUGGER_STATE.take_interruptedthread() else {
    return;
  };
  // Safety: 登记指针为活跃状态（登记方契约）；take 后本侧独占。`resume` 为
  // LuaState 的 unsafe 方法（from 实参位收 C 侧 NULL 哨兵）。
  // FFI: c-API 要求 NULL
  unsafe { state_mut(interruptedthread).resume(null_mut(), 0) };
}

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_debugger_yield(l: *mut LuaState) -> bool {
  let breakhits = CONFORMANCE_DEBUGGER_STATE.breakhits.load(Ordering::SeqCst);
  assert_eq!(breakhits % 2, 1);

  checkstack(l, LUA_MINSTACK);

  match breakhits {
    1 => debugger_yield_break1(l),
    3 => assert_local(l, 0, 1, b"a", 6),
    5 => assert_local(l, 1, 1, b"a", 7),
    7 => assert_local(l, 1, 1, b"a", 8),
    9 => assert_local(l, 1, 1, b"a", 9),
    13 => assert_nil_local_a(l),
    15 => assert_level2_locals(l),
    _ => {}
  }

  resume_interrupted_thread();
  false
}
