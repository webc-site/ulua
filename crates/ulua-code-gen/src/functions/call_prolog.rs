use core::ptr::null;

use ulua_vm::{
  functions::{lua_d_grow_ci::lua_d_grow_ci, lua_v_tryfunc_tm::lua_v_tryfunc_tm},
  macros::lua_d_checkstackfornewci::lua_d_checkstackfornewci,
  records::{call_info::CallInfo, closure::Closure, lua_state::LuaState},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn call_prolog(
  l: *mut LuaState,
  ra: *mut TValue,
  mut argtop: StkId,
  nresults: i32,
) -> *mut Closure {
  // 契约: `l` 为存活 `LuaState`，`ra`/`argtop` 为 VM 调用协议下合法的栈槽（`ra` 经
  // `is_function()`/`lua_v_tryfunc_tm` 保证为可调用值），故 `clvalue!(ra)` 得到存活 `Closure`。
  // `incr_ci` 返回落在预增长 CallInfo 数组内的有效槽；仅写入 `(*ci)` 各字段并把 `l.base/l.top`
  // 指向 ra/argtop 处的栈内存（随 state 存活），单线程串行无 &mut 别名。
  if unsafe { !(*ra).is_function() } {
    // Safety: 依契约; __call 元方法就地改写活槽 ra, argtop 前移一参数位。
    unsafe {
      lua_v_tryfunc_tm(l, ra);
      argtop = argtop.add(1);
    };
  }

  // Safety: 依契约; 守卫后 ra 必为存活可调用闭包。
  let ccl = unsafe { (*ra).as_closure_ptr() };

  // Safety: 依契约; ci 数组内界, func/base/top 赋值为界内 StkId 算术, base/top 回写活 state。
  unsafe {
    let ci = incr_ci(l);
    (*ci).func = ra;
    (*ci).base = ra.add(1);
    (*ci).top = argtop.add((*ccl).stacksize as usize);
    (*ci).savedpc = null();
    (*ci).flags = 0;
    (*ci).nresults = nresults;
    (*l).base = (*ci).base;
    (*l).top = argtop;
  }

  // Safety: 依契约; 新帧栈空间检查(stacksize 为活闭包字段)。
  unsafe { lua_d_checkstackfornewci(l, (*ccl).stacksize as i32) };

  ccl
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
///
/// C++ `++L->ci`：无 condhardstacktests 分支（与 VM 的 `incr_ci!` 宏语义不同），
/// 供 call_prolog / call_fallback 复用。
pub(crate) unsafe fn incr_ci(l: *mut LuaState) -> *mut CallInfo {
  // Safety: `l` 为存活 `LuaState`（调用方 call_prolog/call_fallback 依 VM 协议保证）；`ci`/`end_ci`
  // 框定一段已分配的 CallInfo 数组，`ci == end_ci` 时 `lua_d_grow_ci` 重分配并刷新 `l.ci`，否则
  // `ci.add(1)` 仍在界内，故返回的 `l.ci` 恒指向存活 CallInfo，仅读写裸指针字段。
  unsafe {
    if (*l).ci == (*l).end_ci {
      lua_d_grow_ci(l);
    } else {
      (*l).ci = (*l).ci.add(1);
    }

    (*l).ci
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn call_prolog_export(
  l: *mut LuaState,
  ra: *mut TValue,
  argtop: StkId,
  nresults: i32,
) -> *mut Closure {
  // Safety: `extern "C-unwind"` FFI 壳，按 Lua/C API 契约由宿主 VM 传入存活的 `LuaState` 与合法
  // 栈槽 `ra`/`argtop`；本行仅原样转发给 `call_prolog`，其 unsafe 前置条件即由上述宿主约定满足。
  unsafe { call_prolog(l, ra, argtop, nresults) }
}
