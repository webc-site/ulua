use core::{
  ffi::{c_char, c_int},
  ptr::null,
};

use crate::{
  functions::{
    index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi, lua_v_tostring::lua_v_tostring,
  },
  macros::{
    lua_c_check_gc::luaC_checkGC, svalue::svalue, tsvalue::tsvalue, ttisstring::ttisstring,
  },
  records::lua_state::lua_State,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_tolstring(l: *mut lua_State, idx: c_int, len: *mut usize) -> *const c_char {
  unsafe {
    let mut o: StkId = index2addr(l, idx);

    if !ttisstring!(o) {
      lua_c_threadbarrier_lapi(l);
      if lua_v_tostring(l, o) == 0 {
        if !len.is_null() {
          *len = 0;
        }
        return null();
      }
      luaC_checkGC!(l);
      o = index2addr(l, idx);
    }

    if !len.is_null() {
      *len = (*tsvalue!(o)).len as usize;
    }

    svalue!(o)
  }
}
