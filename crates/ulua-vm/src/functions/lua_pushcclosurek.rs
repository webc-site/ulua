//! Node: `cxx:Function:Luau.VM:VM/src/lapi.cpp:742:lua_pushcclosurek`
//! Source: `VM/src/lapi.cpp:742-760` (hand-ported)

use core::ffi::{c_char, c_int};

use crate::{
  functions::{
    getcurrenv::getcurrenv, lua_concat::lua_c_threadbarrier_lapi,
    lua_f_new_cclosure::luaF_newCclosure,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, api_incr_top::api_incr_top,
    iswhite::iswhite, lua_c_check_gc::luaC_checkGC, setclvalue::setclvalue, setobj_2_n::setobj2n,
  },
  records::gc_object::GCObject,
  type_aliases::{
    lua_c_function::LuaCFunction, lua_continuation::LuaContinuation, lua_state::lua_State,
    t_value::TValue,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_pushcclosurek(
  l: *mut lua_State,
  r#fn: LuaCFunction,
  debugname: *const c_char,
  mut nup: c_int,
  cont: LuaContinuation,
) {
  unsafe {
    api_check!(l, r#fn.is_some());
    api_check!(l, nup >= 0);
    luaC_checkGC!(l);
    lua_c_threadbarrier_lapi(l);
    api_checknelems!(l, nup);

    let cl = luaF_newCclosure(l, nup, getcurrenv(l));
    let cc = core::ptr::addr_of_mut!((*cl).inner.c);
    (*cc).f = r#fn;
    (*cc).cont = cont;
    (*cc).debugname = debugname;

    (*l).top = (*l).top.sub(nup as usize);
    let upvals = core::ptr::addr_of_mut!((*cc).upvals) as *mut TValue;
    while nup > 0 {
      nup -= 1;
      setobj2n!(l, upvals.add(nup as usize), (*l).top.add(nup as usize));
    }

    setclvalue!(l, (*l).top, cl);
    ulua_common::LUAU_ASSERT!(iswhite!(cl as *mut GCObject));
    api_incr_top!(l);
  }
}
