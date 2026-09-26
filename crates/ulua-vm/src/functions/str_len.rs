use crate::{
  functions::{lua_l_checklstring::lua_l_checklstring, lua_pushinteger::lua_pushinteger},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn str_len(l: *mut LuaState) -> i32 {
  unsafe {
    let mut len: usize = 0;
    lua_l_checklstring(l, 1, &mut len);
    lua_pushinteger(l, len as i32);
    1
  }
}
