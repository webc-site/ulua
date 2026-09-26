use crate::{
  functions::{
    lua_l_checkany::lua_l_checkany, lua_pushstring::lua_pushstring, lua_type::lua_type,
    lua_typename::lua_typename,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn lua_b_type(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checkany(l, 1);
    // resulting name doesn't differentiate between userdata types
    let t = lua_type(l, 1);
    let name = lua_typename(l, t);
    lua_pushstring(l, name);
    1
  }
}
