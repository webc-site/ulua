use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_getreadonly::lua_getreadonly, lua_l_checktype::lua_l_checktype,
    lua_pushboolean::lua_pushboolean,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn tisfrozen(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);

    lua_pushboolean(l, lua_getreadonly(l, 1));

    1
  }
}
