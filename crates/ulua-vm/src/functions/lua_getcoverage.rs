use core::{ffi::c_void, ptr::addr_of};

use crate::{
  functions::{getcoverage::getcoverage, getmaxline::getmaxline, lua_a_toobject::lua_a_toobject},
  macros::{api_check::api_check, lua_m_freearray::luaM_freearray, lua_m_newarray::luaM_newarray},
  records::{closure::LClosure, lua_state::LuaState},
  type_aliases::{lua_coverage::LuaCoverage, t_value::TValue},
};

/// # Safety
/// `l` 必须指向存活 `LuaState` 且 `funcindex` 处为 Lua（非 C）闭包 TValue（api_check 仅 debug 兜底）；
/// `context` 与 `callback` 型别约定须匹配——callback 收到按 `l` 分配器申请的行缓冲裸指针，函数返回前即被
/// luaM_freearray! 释放，被调方不得留存该指针。对应 cpp ldebug.cpp:590。
pub unsafe fn lua_getcoverage(
  l: *mut LuaState,
  funcindex: i32,
  context: *mut c_void,
  callback: LuaCoverage,
) {
  // Safety: 契约保证 `L` 存活、funcindex 处为 Lua 闭包可读，`buffer..buffer+size` 与原型指令位图字节数匹配且可写
  unsafe {
    let func: *const TValue = lua_a_toobject(l, funcindex);
    api_check!(
      l,
      (*func).is_function() && (*(*func).as_closure_ptr()).is_c == 0
    );

    let cl = (*func).as_closure_ptr();
    let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
    let p = (*lcl).p;

    let size = getmaxline(p) as usize + 1;
    if size == 0 {
      return;
    }

    let buffer = luaM_newarray!(l, size, i32, 0);

    getcoverage(p, 0, buffer, size, context, callback);

    luaM_freearray!(l, buffer as *mut c_void, size, i32, 0);
  }
}
