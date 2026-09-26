use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checktype::lua_l_checktype, lua_l_optinteger::lua_l_optinteger, lua_objlen::lua_objlen,
    lua_pushnil::lua_pushnil, lua_rawgeti::lua_rawgeti, lua_rawseti::lua_rawseti,
    moveelements::moveelements,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn tremove(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);
    let n = lua_objlen(l, 1);
    let pos = lua_l_optinteger(l, 2, n);

    if !(1 <= pos && pos <= n) {
      return 0;
    }

    lua_rawgeti(l, 1, pos);

    moveelements(l, 1, 1, pos + 1, n, pos, false);

    lua_pushnil(l);
    lua_rawseti(l, 1, n);

    1
  }
}

lua_lib_fn!(pub fn tremove, tremove_arm);
