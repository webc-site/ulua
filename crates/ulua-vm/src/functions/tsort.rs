use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_g_readonlyerror::lua_g_readonlyerror, lua_h_getn::lua_h_getn,
    lua_l_checktype::lua_l_checktype, lua_settop::lua_settop, lua_v_lessthan::lua_v_lessthan,
    sort_func::sort_func, sort_rec::sort_rec,
  },
  macros::{hvalue::hvalue, lua_isnoneornil::lua_isnoneornil},
  type_aliases::{lua_state::lua_State, sort_predicate::SortPredicate},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_tsort"))]
pub(crate) unsafe extern "C-unwind" fn tsort(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);

    let t = hvalue!((*l).base);
    let n = lua_h_getn(t);

    if (*t).readonly != 0 {
      lua_g_readonlyerror(l);
    }

    let mut pred: SortPredicate = Some(lua_v_lessthan);
    if !lua_isnoneornil!(l, 2) {
      lua_l_checktype(l, 2, LuaType::Function as c_int);
      pred = Some(sort_func);
    }
    lua_settop(l, 2);

    if n > 0 {
      sort_rec(l, t, 0, n - 1, n, pred);
    }
    0
  }
}
