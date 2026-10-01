use core::cmp::Ordering::{Equal, Less};

use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn math_clamp(l: *mut LuaState) -> i32 {
  unsafe {
    let v = (*l).check_number(1);
    let min = (*l).check_number(2);
    let max = (*l).check_number(3);

    (*l).arg_check(
      matches!(min.partial_cmp(&max), Some(Less | Equal)),
      3,
      "max must be greater than or equal to min",
    );

    let r = if v < min { min } else { v };
    let r = if r > max { max } else { r };

    (*l).push_number(r);
    1
  }
}

lua_lib_fn!(pub fn math_clamp, math_clamp_arm);
