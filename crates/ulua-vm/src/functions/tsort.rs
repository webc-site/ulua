use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_g_readonlyerror::check_writable, lua_h_getn::lua_h_getn, lua_l_checktype::lua_l_checktype,
    lua_settop::lua_settop, lua_v_lessthan::lua_v_lessthan, sort_func::sort_func,
    sort_rec::sort_rec,
  },
  macros::lua_isnoneornil::lua_isnoneornil,
  records::lua_state::LuaState,
  type_aliases::sort_predicate::SortPredicate,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn tsort(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);

    let t = (*(*l).base).as_table_ptr();
    let n = lua_h_getn(t);

    check_writable(l, t);

    let mut pred: SortPredicate = Some(lua_v_lessthan);
    if !lua_isnoneornil!(l, 2) {
      lua_l_checktype(l, 2, LuaType::Function as i32);
      pred = Some(sort_func);
    }
    lua_settop(l, 2);

    if n > 0 {
      sort_rec(l, t, 0, n - 1, n, pred);
    }
    0
  }
}
