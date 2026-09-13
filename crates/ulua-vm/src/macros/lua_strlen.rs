use core::ffi::c_int;

use crate::{functions::lua_objlen::lua_objlen, records::lua_state::lua_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn lua_strlen(l: *mut lua_State, idx: c_int) -> usize {
  unsafe { lua_objlen(l, idx) as usize }
}
