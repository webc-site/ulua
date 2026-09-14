//! `aux_upvalue` — resolve the n-th upvalue slot in a function TValue.
//! C++ source: `VM/src/lapi.cpp:1487`
//!
//! Returns the C-string name of the upvalue (or `""`) and writes `*val` to the
//! corresponding `TValue*`.  Returns `NULL` when `fi` is not a function or `n`
//! is out of range.

use core::{
  ffi::{c_char, c_int},
  ptr::null,
};

use crate::{
  enums::lua_type::LuaType,
  macros::{clvalue::clvalue, getstr::getstr, ttisfunction::ttisfunction},
  records::up_val::UpVal,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// `static const char* aux_upvalue(StkId fi, int n, TValue** val)`
pub(crate) unsafe fn aux_upvalue(fi: StkId, n: c_int, val: *mut *mut TValue) -> *const c_char {
  unsafe {
    if !ttisfunction!(fi) {
      return null();
    }
    let f = &*clvalue!(fi);

    if f.is_c != 0 {
      // C closure — upvalues stored inline in `c.upvals`
      if !(1 <= n && n <= f.nupvalues as c_int) {
        return null();
      }
      *val = f.inner.c.upvals.as_ptr().add((n - 1) as usize) as *mut TValue;
      c"".as_ptr()
    } else {
      // Lua closure
      let p = f.inner.l.p;
      if !(1 <= n && n <= (*p).nups as c_int) {
        return null();
      }
      // uprefs is a flexible array — use pointer arithmetic
      let r: *mut TValue = f.inner.l.uprefs.as_ptr().add((n - 1) as usize) as *mut TValue;
      // ttisupval(r): ttype(r) == LUA_TUPVAL
      *val = if (*r).tt() == LuaType::Upval as c_int {
        // upvalue(r)->v
        (*(core::ptr::addr_of!((*(*r).value.gc).uv) as *const UpVal)).v
      } else {
        r
      };
      if !(1 <= n && n <= (*p).sizeupvalues) {
        return c"".as_ptr();
      }
      getstr(*(*p).upvalues.add((n - 1) as usize))
    }
  }
}
