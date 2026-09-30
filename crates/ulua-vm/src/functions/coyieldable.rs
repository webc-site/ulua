use crate::{
  functions::lua_isyieldable::lua_isyieldable, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn coyieldable(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).push_boolean(lua_isyieldable(l) != 0);
    1
  }
}

lua_lib_fn!(pub fn coyieldable, coyieldable_arm);
