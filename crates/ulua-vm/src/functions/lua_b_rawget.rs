use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checkany::lua_l_checkany, lua_l_checktype::lua_l_checktype, lua_rawget::lua_rawget,
    lua_settop::lua_settop,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_b_rawget(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);
    lua_l_checkany(l, 2);
    lua_settop(l, 2);
    lua_rawget(l, 1);
    1
  }
}

lua_lib_fn!(pub fn lua_b_rawget, lua_b_rawget_arm);
