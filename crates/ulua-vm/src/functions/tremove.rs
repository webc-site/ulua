use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_optinteger::lua_l_optinteger, lua_rawgeti::lua_rawgeti, lua_rawseti::lua_rawseti,
    moveelements::moveelements,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn tremove(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);
    let n = (*l).obj_len(1) as i32;
    let pos = lua_l_optinteger(&mut *l, 2, n);

    if !(1 <= pos && pos <= n) {
      return 0;
    }

    lua_rawgeti(&mut *l, 1, pos);

    moveelements(l, 1, 1, pos + 1, n, pos, false);

    (*l).push_nil();
    lua_rawseti(&mut *l, 1, n);

    1
  }
}

lua_lib_fn!(pub fn tremove, tremove_arm);
