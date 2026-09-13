use core::ffi::c_int;

use crate::{
  macros::{api_check::api_check, setnilvalue::setnilvalue},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_settop")]
pub unsafe fn lua_settop(l: *mut lua_State, idx: c_int) {
  unsafe {
    if idx >= 0 {
      api_check!(l, idx as isize <= (*l).stack_last.offset_from((*l).base));
      while (*l).top < (*l).base.add(idx as usize) {
        setnilvalue!((*l).top);
        (*l).top = (*l).top.add(1);
      }
      (*l).top = (*l).base.add(idx as usize);
    } else {
      api_check!(l, -(idx + 1) as isize <= (*l).top.offset_from((*l).base));
      (*l).top = (*l).top.offset((idx + 1) as isize); // `subtract' index (index is negative)
    }
  }
}
