use core::{ffi::c_int, mem::size_of, ptr::read_unaligned};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  macros::{
    bufvalue::bufvalue, checkoutofbounds::checkoutofbounds, nvalue::nvalue, setlvalue::setlvalue,
    ttisbuffer::ttisbuffer, ttisnumber::ttisnumber,
  },
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn luau_f_bufferreadlong(
  _l: *mut lua_State,
  res: StkId,
  arg0: *mut TValue,
  nresults: c_int,
  args: StkId,
  nparams: c_int,
) -> c_int {
  unsafe {
    if !LUAU_BIG_ENDIAN && nparams >= 2 && nresults <= 1 && ttisbuffer!(arg0) && ttisnumber!(args) {
      let offset = nvalue!(args) as c_int;

      let len = bufvalue!(arg0).len as usize;
      if checkoutofbounds(offset, len, size_of::<i64>()) {
        return -1;
      }

      let val: i64 = {
        let src = bufvalue!(arg0).data.as_ptr().add(offset as usize) as *const i64;
        read_unaligned(src)
      };

      setlvalue!(res, val);
      return 1;
    }

    -1
  }
}

pub use luau_f_bufferreadlong as luauF_bufferreadlong;
