use core::{ffi::c_void, ptr::addr_of};

use crate::{
  functions::{getcoverage::getcoverage, getmaxline::getmaxline, lua_a_toobject::lua_a_toobject},
  macros::api_check::api_check,
  records::{closure::LClosure, lua_state::LuaState},
  type_aliases::{lua_coverage::LuaCoverage, t_value::TValue},
};

/// # Safety
/// `l` 必须指向存活 `LuaState` 且 `funcindex` 处为 Lua（非 C）闭包 TValue（api_check 仅 debug 兜底）；
/// `context` 与 `callback` 型别约定须匹配——callback 收到按本帧 `Vec` 分配的行缓冲共享切片，函数
/// 返回前即随 `Vec` 释放，被调方不得留存该借用、不得越权改写。对应 cpp ldebug.cpp:590。
///
/// review.md §10：回调 `function` 形参为原生 `Option<&[u8]>` 串体窗（见 `LuaCoverage`）。
pub unsafe fn lua_getcoverage(
  l: *mut LuaState,
  funcindex: i32,
  context: *mut c_void,
  callback: LuaCoverage,
) {
  // SAFETY: 契约保证 `l` 存活、funcindex 处为 Lua 闭包可读；行缓冲出参已折叠为帧内
  // `Vec<i32>`（§3 出参惯用化），不再经 luaM arena 转手，其裸指针仅在 callback 调用窗口内有效。
  unsafe {
    let func: *const TValue = lua_a_toobject(&*l, funcindex);
    api_check!(
      l,
      (*func).is_function() && (*(*func).as_closure_ptr()).is_c == 0
    );

    let cl = (*func).as_closure_ptr();
    let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
    let p = (*lcl).p;

    // 契约保证闭包 `p` 指向存活 Proto；降共享引用透传给只读内部函数
    let size = getmaxline(&*p) as usize + 1;
    if size == 0 {
      return;
    }

    let mut buffer = vec![0i32; size];

    getcoverage(&*p, 0, &mut buffer, context, callback);
  }
}
