use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn math_lerp(l: *mut LuaState) -> i32 {
  unsafe {
    let a = lua_l_checknumber(l, 1);
    let b = lua_l_checknumber(l, 2);
    let t = lua_l_checknumber(l, 3);

    let r = if t == 1.0 { b } else { a + (b - a) * t };

    lua_pushnumber(l, r);
    1
  }
}

lua_lib_fn!(pub fn math_lerp, math_lerp_arm);
