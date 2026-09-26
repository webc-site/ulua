use core::{ffi::c_int, mem::zeroed, ptr::null_mut, sync::atomic::Ordering};

use ulua_vm::{
  functions::{
    lua_checkstack::lua_checkstack, lua_getargument::lua_getargument, lua_getinfo::lua_getinfo,
    lua_getlocal::lua_getlocal, lua_getupvalue::lua_getupvalue, lua_resume::lua_resume,
  },
  macros::{
    lua_isnil::lua_isnil, lua_minstack::LUA_MINSTACK, lua_pop::lua_pop,
    lua_tointeger::lua_tointeger,
  },
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

use crate::common::{
  functions::{cstr::cstr, cstr_text::cstr_raw},
  records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE,
};

/// 校验栈顶整数值并弹出（cpp `assertStackInteger` lambda）。
///
/// # Safety
///
/// `l` 为活跃 VM 状态且栈顶持有一个 integer 值。
unsafe fn assert_stack_integer(l: *mut LuaState, value: c_int) {
  // Safety: 本函数 `# Safety` 契约（由调用方自入口转承）保证 `l` 活跃且栈顶可读。
  unsafe {
    assert_eq!(lua_tointeger!(l, -1), value);
    lua_pop(l, 1);
  }
}

/// 校验 level/slot 处局部变量的名字与值（cpp `assertLocal` lambda）。
///
/// # Safety
///
/// `l` 为活跃 VM 状态，给定 `level`/`slot` 处存在名为 `name`、值为整数 `value` 的局部变量。
unsafe fn assert_local(l: *mut LuaState, level: c_int, slot: c_int, name: &[u8], value: c_int) {
  // Safety: 本函数 `# Safety` 契约保证 getlocal 可用且返回值非空。
  unsafe {
    let local = lua_getlocal(l, level, slot);
    assert!(!local.is_null());
    assert_eq!(cstr_raw(local), name);
    assert_stack_integer(l, value);
  }
}

/// breakhits==1 分支：校验两个实参、局部变量 b 与闭包 upvalue a（cpp 首命中）。
///
/// # Safety
///
/// `l` 为活跃 VM 状态；当前栈帧有两个整型实参 50/42、局部变量 `b`，且被调闭包带
/// upvalue `a`（值 5）。
unsafe fn debugger_yield_break1(l: *mut LuaState) {
  // Safety: 本函数 `# Safety` 契约（由入口转承）保证 `l` 活跃、实参与局部变量在位。
  unsafe {
    assert_ne!(0, lua_getargument(l, 0, 1));
    assert_stack_integer(l, 50);

    assert_ne!(0, lua_getargument(l, 0, 2));
    assert_stack_integer(l, 42);

    assert_local(l, 0, 1, b"b", 50);
  }

  // Safety: 同上；getinfo 的 `f` 选项只填 ar.source 一类字段且本分支不读取，
  // upvalue 校验对栈的影响以 `lua_pop(l, 2)` 收尾配平。
  unsafe {
    let mut ar: LuaDebug = zeroed();
    lua_getinfo(l, 0, cstr(b"f\0"), &mut ar);

    let upvalue = lua_getupvalue(l, -1, 1);
    assert!(!upvalue.is_null());
    assert_eq!(cstr_raw(upvalue), b"a");
    assert_eq!(lua_tointeger!(l, -1), 5);
    lua_pop(l, 2);
  }
}

/// breakhits==13 分支：局部变量 `a` 存在且为 nil。
///
/// # Safety
///
/// `l` 为活跃 VM 状态且 level 0 的 slot 1 处是值为 nil 的局部变量 `a`。
unsafe fn assert_nil_local_a(l: *mut LuaState) {
  // Safety: 本函数 `# Safety` 契约（由入口转承）。
  unsafe {
    let local = lua_getlocal(l, 0, 1);
    assert!(!local.is_null());
    assert_eq!(cstr_raw(local), b"a");
    assert!(lua_isnil!(l, -1));
    lua_pop(l, 1);
  }
}

/// breakhits==15 分支：level 2 的 slot 1 是 `x`、slot 2 越界为空。
///
/// # Safety
///
/// `l` 为活跃 VM 状态且 level 2 处至少有一个局部变量。
unsafe fn assert_level2_locals(l: *mut LuaState) {
  // Safety: 本函数 `# Safety` 契约（由入口转承）。
  unsafe {
    let x = lua_getlocal(l, 2, 1);
    assert!(!x.is_null());
    assert_eq!(cstr_raw(x), b"x");
    lua_pop(l, 1);

    let a1 = lua_getlocal(l, 2, 2);
    assert!(a1.is_null());
  }
}

/// 若有用例登记了待恢复线程则取出并 resume（cpp debuggerHook 收尾段）。
///
/// # Safety
///
/// 登记进 `interruptedthread` 的指针（若有）为存活的 VM 状态。
unsafe fn resume_interrupted_thread() {
  let Some(interruptedthread) = CONFORMANCE_DEBUGGER_STATE.take_interruptedthread() else {
    return;
  };
  // Safety: 本函数 `# Safety` 契约保证登记指针为活跃状态；take 后本侧独占。
  // FFI: c-API 要求 NULL
  unsafe { lua_resume(interruptedthread, null_mut(), 0) };
}

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_debugger_yield(l: *mut LuaState) -> bool {
  let breakhits = CONFORMANCE_DEBUGGER_STATE.breakhits.load(Ordering::SeqCst);
  assert_eq!(breakhits % 2, 1);

  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、
  // Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与
  // C ABI 的前置条件。
  unsafe { lua_checkstack(l, LUA_MINSTACK) };

  // Safety: 同上；各分支即 cpp debuggerHook 的行为用例，前置条件由各 helper 的
  // `# Safety` 契约在此收口。
  match breakhits {
    1 => unsafe { debugger_yield_break1(l) },
    3 => unsafe { assert_local(l, 0, 1, b"a", 6) },
    5 => unsafe { assert_local(l, 1, 1, b"a", 7) },
    7 => unsafe { assert_local(l, 1, 1, b"a", 8) },
    9 => unsafe { assert_local(l, 1, 1, b"a", 9) },
    13 => unsafe { assert_nil_local_a(l) },
    15 => unsafe { assert_level2_locals(l) },
    _ => {}
  }

  // Safety: swap 为安全原子操作；resume 的前置条件由
  // [`resume_interrupted_thread`] 的 `# Safety` 契约收口。
  unsafe { resume_interrupted_thread() };
  false
}
