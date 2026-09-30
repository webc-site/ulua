use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_g_readonlyerror::check_writable, lua_h_getn::lua_h_getn, lua_v_lessthan::lua_v_lessthan,
    sort_func::sort_func, sort_rec::sort_rec,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
  type_aliases::sort_predicate::SortPredicate,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn tsort(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);

    let t = (*(*l).base).as_table_ptr();
    let n = lua_h_getn(t);

    check_writable(l, t);

    let mut pred: SortPredicate = Some(lua_v_lessthan);
    if !(*l).is_none_or_nil(2) {
      (*l).check_type(2, LuaType::Function);
      pred = Some(sort_func);
    }
    (*l).set_top(2);

    if n > 0 {
      sort_rec(l, t, 0, n - 1, n, pred);
    }
    0
  }
}

lua_lib_fn!(pub fn tsort, tsort_arm);
