use core::ptr::addr_of;

use crate::{
  functions::{
    getnextline::getnextline, lua_a_toobject::lua_a_toobject, lua_g_breakpoint::lua_g_breakpoint,
  },
  macros::api_check::api_check,
  records::{closure::LClosure, lua_state::LuaState, proto::Proto},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 必须指向存活 `LuaState`，且 `funcindex` 栈槽处的值必须是 Lua（非 C）闭包 TValue——api_check 仅 debug
/// 兜底，release 下误传 C 闭包会按 LClosure 布局误读 inner 联合；其 `p` 指向的 Proto 须存活且 code/abslineinfo
/// 与 sizecode 一致（`lua_g_breakpoint` 按 pc 下标读写断言位图）。对应 cpp ldebug.cpp:543。
pub unsafe fn lua_breakpoint(l: *mut LuaState, funcindex: i32, line: i32, enabled: i32) -> i32 {
  // Safety: 契约保证 `l` 存活、funcindex 处为 Lua 闭包（api_check 兜底），pc 表索引经 code 长度上界检查
  unsafe {
    let func: *const TValue = lua_a_toobject(l, funcindex);
    api_check!(
      l,
      (*func).is_function() && (*(*func).as_closure_ptr()).is_c == 0
    );

    let cl = (*func).as_closure_ptr();
    let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
    let p: *mut Proto = (*lcl).p;

    let target = getnextline(p, line);

    if target != -1 {
      lua_g_breakpoint(l, p, target, enabled != 0);
    }

    target
  }
}
