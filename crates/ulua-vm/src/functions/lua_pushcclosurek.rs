//! Source: `VM/src/lapi.cpp:742-760` (hand-ported)

use core::{ffi::c_char, ptr::addr_of_mut};

use crate::{
  functions::{
    c_slice, c_slice_mut, ensure_stack::ensure_stack, getcurrenv::getcurrenv,
    lapi_barrier::lua_c_threadbarrier_lapi, lua_f_new_cclosure::lua_f_new_cclosure,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, api_incr_top::api_incr_top,
    iswhite::iswhite, lua_c_check_gc::lua_c_check_gc, setclvalue::setclvalue, setobj_2_n::setobj2n,
  },
  records::{gc_object::GCObject, lua_state::LuaState},
  type_aliases::{
    lua_c_function::LuaCFunction, lua_continuation::LuaContinuation, t_value::TValue,
  },
};

/// # Safety
/// `l` 指向存活 `LuaState`；`fn` 非空且遵循 Lua C 函数约定（cpp lapi.cpp:783 `api_check(fn)`）；
/// `nup >= 0` 且栈顶已压入 `nup` 个可捕获上值（`api_checknelems`）；`debugname` 为空或须在
/// 闭包存活期内保持有效的 NUL 串；`cont` 为可在可 yield 路径安全调用的回调。cpp lapi.cpp:781.
pub unsafe fn lua_pushcclosurek(
  l: *mut LuaState,
  r#fn: LuaCFunction,
  debugname: *const c_char,
  nup: i32,
  cont: LuaContinuation,
) {
  // Safety: 契约保证 `l` 为存活调用帧且自栈顶起 nup 个 upvalue 槽可读，闭包按 nup 分配、复制与 GC 记账在界内
  unsafe {
    api_check!(l, r#fn.is_some());
    api_check!(l, nup >= 0);
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);
    api_checknelems!(l, nup);

    let cl = lua_f_new_cclosure(l, nup, getcurrenv(l));
    let cc = addr_of_mut!((*cl).inner.c);
    (*cc).f = r#fn;
    (*cc).cont = cont;
    (*cc).debugname = debugname;

    (*l).top = (*l).top.sub(nup as usize);
    let upvals = addr_of_mut!((*cc).upvals) as *mut TValue;
    // 出栈后自 top 起 nup 槽即待捕获值窗口：dst（闭包 upvals 堆块）与 src（栈窗口）
    // 不相交，c_slice 配对逐格 setobj2n 与 cpp 倒序写等价，免 `top.add(k)` 裸走查
    for (dst, src) in c_slice_mut(upvals, nup as usize)
      .iter_mut()
      .zip(c_slice((*l).top, nup as usize))
    {
      setobj2n!(l, dst as *mut TValue, src as *const TValue);
    }

    setclvalue!(l, (*l).top, cl);
    ulua_common::LUAU_ASSERT!(iswhite!(cl as *mut GCObject));
    api_incr_top!(l);
  }
}
