use crate::{
  enums::tms::TMS,
  functions::{call_t_mres::call_t_mres, lua_t_gettmbyobj::lua_t_gettmbyobj},
  macros::ttisnil::ttisnil,
  type_aliases::{lua_state::LuaState, stk_id::StkId, t_value::TValue},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn call_bin_tm(
  l: *mut LuaState,
  p1: *const TValue,
  p2: *const TValue,
  res: StkId,
  event: TMS,
) -> i32 {
  unsafe {
    let mut tm = lua_t_gettmbyobj(l, p1, event); // try first operand
    if ttisnil!(tm) {
      tm = lua_t_gettmbyobj(l, p2, event); // try second operand
    }
    if ttisnil!(tm) {
      return 0;
    }
    call_t_mres(l, res, tm, p1, p2);
    1
  }
}
