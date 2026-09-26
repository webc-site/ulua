use crate::{
  functions::{lua_isyieldable::lua_isyieldable, lua_pushboolean::lua_pushboolean},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn coyieldable(l: *mut LuaState) -> i32 {
  unsafe {
    lua_pushboolean(l, lua_isyieldable(l));
    1
  }
}
