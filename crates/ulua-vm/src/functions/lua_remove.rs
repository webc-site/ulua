use core::ffi::c_int;

use crate::{
  functions::index_2_addr::index2addr,
  macros::{api_check::api_check, setobj_2_s::setobj2s},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_remove(l: *mut lua_State, idx: c_int) {
  unsafe {
    let p: StkId = index2addr(l, idx);

    // api_checkvalidindex macro expands to a reference to crate::records::lobject::luaO_nilobject,
    // but that module isn't available in this crate layout. Replicate its behavior directly.
    api_check!(l, !p.is_null());

    let mut p = p;
    while p.offset(1) < (*l).top {
      p = p.offset(1);
      setobj2s!(l, p.offset(-1), p);
    }
    (*l).top = (*l).top.offset(-1);
  }
}
