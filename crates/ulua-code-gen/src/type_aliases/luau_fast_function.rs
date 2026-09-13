use ulua_vm::type_aliases::{stk_id::StkId, t_value::TValue};

use crate::type_aliases::lua_state::lua_State;

pub type LuauFastFunction = Option<
  unsafe extern "C-unwind" fn(
    l: *mut lua_State,
    res: StkId,
    arg0: *mut TValue,
    nresults: i32,
    args: StkId,
    nparams: i32,
  ) -> i32,
>;
