use core::ffi::c_int;

use crate::type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn luau_f_missing(
  _l: *mut lua_State,
  _res: StkId,
  _arg0: *mut TValue,
  _nresults: c_int,
  _args: StkId,
  _nparams: c_int,
) -> c_int {
  -1
}
