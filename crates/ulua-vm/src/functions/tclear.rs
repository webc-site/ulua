use crate::{
  enums::lua_type::LuaType,
  functions::{lua_g_readonlyerror::check_writable, lua_h_clear::lua_h_clear},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn tclear(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);

    let tt = (*(*l).base).as_table_ptr();

    check_writable(l, tt);

    lua_h_clear(tt);
    0
  }
}

lua_lib_fn!(pub fn tclear, tclear_arm);
