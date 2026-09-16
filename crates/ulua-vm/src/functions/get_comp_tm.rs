use core::ptr::null;

use crate::{
  enums::tms::TMS,
  functions::lua_o_rawequal_obj::luaO_rawequalObj,
  macros::fasttm::fasttm,
  records::lua_table::LuaTable,
  type_aliases::{lua_state::LuaState, t_value::TValue},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn get_comp_tm(
  l: *mut LuaState,
  mt1: *mut LuaTable,
  mt2: *mut LuaTable,
  event: TMS,
) -> *const TValue {
  unsafe {
    let tm1 = fasttm(l, mt1, event as i32);

    if tm1.is_null() {
      return null();
    }

    if mt1 == mt2 {
      return tm1;
    }

    let tm2 = fasttm(l, mt2, event as i32);
    if tm2.is_null() {
      return null();
    }

    if luaO_rawequalObj(tm1, tm2) != 0 {
      return tm1;
    }

    null()
  }
}
