use core::{
  ffi::{c_char, c_int},
  ptr::null_mut,
};

use crate::{
  functions::{
    aux_upvalue::aux_upvalue, index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi,
  },
  macros::{api_incr_top::api_incr_top, setobj_2_s::setobj2s},
  records::lua_state::lua_State,
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_getupvalue(l: *mut lua_State, funcindex: c_int, n: c_int) -> *const c_char {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    let mut val: *mut TValue = null_mut();
    let name: *const c_char = aux_upvalue(index2addr(l, funcindex), n, &mut val);

    if !name.is_null() {
      setobj2s!(l, (*l).top, val);
      api_incr_top!(l);
    }

    name
  }
}
