use core::{
  ffi::{c_char, c_int},
  ptr::null,
};

use crate::{
  functions::index_2_addr::index2addr,
  macros::{
    getstr::getstr, lua_s_updateatom::lua_s_updateatom, tsvalue::tsvalue, ttisstring::ttisstring,
  },
  records::t_string::tstring,
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_tostringatom(l: *mut lua_State, idx: c_int, atom: *mut c_int) -> *const c_char {
  unsafe {
    let o: StkId = index2addr(l, idx);

    if !ttisstring!(o) {
      return null();
    }

    let s = tsvalue!(o);
    if !atom.is_null() {
      lua_s_updateatom!(l, s as *mut tstring);
      *atom = (*s).atom as c_int;
    }

    getstr(s)
  }
}
