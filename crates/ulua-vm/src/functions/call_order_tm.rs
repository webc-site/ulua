use crate::{
  enums::tms::TMS,
  functions::{
    call_t_mres::call_t_mres, lua_g_ordererror::luaG_ordererror,
    lua_o_rawequal_obj::luaO_rawequalObj, lua_t_gettmbyobj::lua_t_gettmbyobj,
  },
  macros::{l_isfalse::l_isfalse, ttisnil::ttisnil},
  type_aliases::{lua_state::LuaState, t_value::TValue},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn call_order_tm(
  l: *mut LuaState,
  p1: *const TValue,
  p2: *const TValue,
  event: TMS,
  error: bool,
) -> i32 {
  unsafe {
    let tm1 = lua_t_gettmbyobj(l, p1, event);

    if ttisnil!(tm1) {
      if error {
        luaG_ordererror(l, p1, p2, event);
      }
      return -1;
    }

    let tm2 = lua_t_gettmbyobj(l, p2, event);
    if luaO_rawequalObj(tm1, tm2) == 0 {
      if error {
        luaG_ordererror(l, p1, p2, event);
      }
      return -1;
    }

    call_t_mres(l, (*l).top, tm1, p1, p2);
    if l_isfalse!((*l).top) { 0 } else { 1 }
  }
}

pub use call_order_tm as call_orderTM;
