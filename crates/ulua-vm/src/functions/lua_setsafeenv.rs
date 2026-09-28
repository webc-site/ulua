use crate::{
  functions::index_2_addr::index_2_addr,
  macros::api_check::api_check,
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_setsafeenv(l: *mut LuaState, objindex: i32, enabled: i32) {
  unsafe {
    let o: *const TValue = index_2_addr(l, objindex);
    api_check!(l, (*o).is_table());
    let t: *mut LuaTable = (*o).as_table_ptr();
    (*t).safeenv = if enabled != 0 { 1 } else { 0 };
  }
}
