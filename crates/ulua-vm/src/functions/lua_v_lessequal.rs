use crate::{
  enums::tms::TMS,
  functions::{
    call_order_tm::call_orderTM, lua_g_ordererror::luaG_ordererror, lua_v_strcmp::luaV_strcmp,
  },
  macros::{
    luai_numle::luai_numle, nvalue::nvalue, tsvalue::tsvalue, ttisnumber::ttisnumber,
    ttisstring::ttisstring, ttype::ttype,
  },
  records::lua_state::lua_State,
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_v_lessequal(
  l: *mut lua_State,
  lhs: *const TValue,
  rhs: *const TValue,
) -> i32 {
  unsafe {
    if ttype!(lhs) != ttype!(rhs) {
      luaG_ordererror(l, lhs, rhs, TMS::TmLe);
    } else if ttisnumber!(lhs) {
      luai_numle(nvalue!(lhs), nvalue!(rhs)) as i32
    } else if ttisstring!(lhs) {
      (luaV_strcmp(tsvalue!(lhs), tsvalue!(rhs)) <= 0) as i32
    } else {
      let res = call_orderTM(l, lhs, rhs, TMS::TmLe, false);
      if res != -1 {
        return res;
      }
      let res = call_orderTM(l, rhs, lhs, TMS::TmLt, false);
      if res == -1 {
        luaG_ordererror(l, lhs, rhs, TMS::TmLe);
      }
      (res == 0) as i32
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaV_lessequal")]
pub unsafe extern "C-unwind" fn lua_v_lessequal_export(
  l: *mut lua_State,
  lhs: *const TValue,
  rhs: *const TValue,
) -> i32 {
  unsafe { lua_v_lessequal(l, lhs, rhs) }
}
