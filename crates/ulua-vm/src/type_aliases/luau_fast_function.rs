use core::ffi::c_int;

use crate::type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue};

pub type LuauFastFunction = Option<
  unsafe extern "C-unwind" fn(
    l: *mut lua_State,
    res: StkId,
    arg0: *mut TValue,
    nresults: c_int,
    args: StkId,
    nparams: c_int,
  ) -> c_int,
>;

pub use LuauFastFunction as luau_FastFunction;
