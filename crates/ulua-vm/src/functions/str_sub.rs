use crate::{
  functions::{
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checklstring::lua_l_checklstring,
    lua_l_optinteger::lua_l_optinteger, lua_pushlstring::lua_pushlstring, posrelat::posrelat,
  },
  macros::{lua_lib_fn::lua_lib_fn, lua_pushliteral::lua_pushliteral},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn str_sub(l: *mut LuaState) -> i32 {
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);
    let mut start = posrelat(lua_l_checkinteger(l, 2), len);
    let mut end = posrelat(lua_l_optinteger(l, 3, -1), len);

    if start < 1 {
      start = 1;
    }
    if end > len as i32 {
      end = len as i32;
    }

    if start <= end {
      lua_pushlstring(l, s.add((start - 1) as usize), (end - start + 1) as usize);
    } else {
      lua_pushliteral(l, b"");
    }
    1
  }
}

lua_lib_fn!(pub fn str_sub, str_sub_arm);
