use core::ptr::null;

use crate::{
  functions::gmatch_aux::gmatch_aux_arm, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn gmatch(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_bytes(1);
    (*l).check_bytes(2);
    (*l).set_top(2);
    (*l).push_integer(0);
    (*l).push_c_closure(Some(gmatch_aux_arm), null(), 3);
    1
  }
}

lua_lib_fn!(pub fn gmatch, gmatch_arm);
