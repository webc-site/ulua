use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_g_readonlyerror::check_writable, lua_h_clear::lua_h_clear, lua_l_checktype::lua_l_checktype,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn tclear(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);

    let tt = (*(*l).base).as_table_ptr();

    check_writable(l, tt);

    lua_h_clear(tt);
    0
  }
}
