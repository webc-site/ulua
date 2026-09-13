use core::ffi::c_int;

use crate::{
  functions::lua_d_callint::lua_d_callint,
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

// plain `unsafe fn` (not extern "C"): Lua errors unwind through here via
// panic/catch_unwind, and extern "C" frames abort on unwind
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_d_call(l: *mut lua_State, func: StkId, nresults: c_int) {
  unsafe {
    lua_d_callint(l, func, nresults, false);
  }
}
