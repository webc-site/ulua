use core::ffi::c_int;

use ulua_common::macros::{luau_likely::LUAU_LIKELY, luau_unlikely::LUAU_UNLIKELY};

use crate::{
  enums::tms::TMS,
  functions::{
    call_order_tm::call_orderTM, lua_g_ordererror::luaG_ordererror, lua_v_strcmp::luaV_strcmp,
  },
  macros::{
    luai_numlt::luai_numlt, nvalue::nvalue, tsvalue::tsvalue, ttisnumber::ttisnumber,
    ttisstring::ttisstring, ttype::ttype,
  },
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn lua_v_lessthan(
  l: *mut lua_State,
  lhs: *const TValue,
  rhs: *const TValue,
) -> c_int {
  unsafe {
    if LUAU_UNLIKELY!(ttype!(lhs) != ttype!(rhs)) {
      luaG_ordererror(l, lhs, rhs, TMS::TmLt);
    } else if LUAU_LIKELY!(ttisnumber!(lhs)) {
      luai_numlt(nvalue!(lhs), nvalue!(rhs)) as c_int
    } else if ttisstring!(lhs) {
      if luaV_strcmp(tsvalue!(lhs), tsvalue!(rhs)) < 0 {
        1
      } else {
        0
      }
    } else {
      call_orderTM(l, lhs, rhs, TMS::TmLt, true)
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaV_lessthan")]
pub unsafe extern "C-unwind" fn lua_v_lessthan_export(
  l: *mut lua_State,
  lhs: *const TValue,
  rhs: *const TValue,
) -> c_int {
  unsafe { lua_v_lessthan(l, lhs, rhs) }
}
