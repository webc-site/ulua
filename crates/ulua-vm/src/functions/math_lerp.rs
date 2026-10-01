use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn math_lerp(l: *mut LuaState) -> i32 {
  unsafe {
    let a = (*l).check_number(1);
    let b = (*l).check_number(2);
    let t = (*l).check_number(3);

    let r = if t == 1.0 { b } else { a + (b - a) * t };

    (*l).push_number(r);
    1
  }
}

lua_lib_fn!(pub fn math_lerp, math_lerp_arm);
