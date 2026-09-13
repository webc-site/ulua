use core::ffi::c_int;

use crate::type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue};

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
