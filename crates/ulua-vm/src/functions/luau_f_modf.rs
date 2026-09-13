use core::ffi::c_int;

use crate::{
  macros::{nvalue::nvalue, setnvalue::setnvalue, ttisnumber::ttisnumber},
  type_aliases::{lua_state::LuaState, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn luau_f_modf(
  _l: *mut LuaState,
  res: StkId,
  arg0: *mut TValue,
  nresults: c_int,
  _args: StkId,
  nparams: c_int,
) -> c_int {
  unsafe {
    if nparams >= 1 && nresults <= 2 && ttisnumber!(arg0) {
      let a1 = nvalue!(arg0);
      let fp = a1.fract();
      let ip = a1 - fp;

      setnvalue!(res, ip);
      setnvalue!(res.add(1), fp);
      2
    } else {
      -1
    }
  }
}

pub use luau_f_modf as luauF_modf;
