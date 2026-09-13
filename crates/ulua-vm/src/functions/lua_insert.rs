//! Node: `cxx:Function:Luau.VM:VM/src/lapi.cpp:272:lua_insert`
//! Source: `VM/src/lapi.cpp:272-280` (hand-ported)

use core::{ffi::c_int, ptr::eq};

use crate::{
  functions::{index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi},
  macros::{api_check::api_check, lua_o_nilobject::luaO_nilobject, setobj_2_s::setobj2s},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_insert(l: *mut lua_State, idx: c_int) {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    let p: StkId = index2addr(l, idx);
    api_check!(l, !eq(p, luaO_nilobject));

    let mut q = (*l).top;
    while q > p {
      setobj2s!(l, q, q.sub(1));
      q = q.sub(1);
    }
    setobj2s!(l, p, (*l).top);
  }
}
