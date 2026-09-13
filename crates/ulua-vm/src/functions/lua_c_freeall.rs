use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{c_slice, deletegco::deletegco, lua_m_visitgco::lua_m_visitgco},
  records::{global_state::global_State, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_c_freeall(l: *mut lua_State) {
  unsafe {
    let g: *mut global_State = (*l).global;

    LUAU_ASSERT!(l == (*g).mainthread);

    lua_m_visitgco(l, l as *mut c_void, deletegco as *mut c_void);

    // free 后全部 bucket 应为空
    for &bucket in c_slice((*g).strt.hash, (*g).strt.size as usize) {
      LUAU_ASSERT!(bucket.is_null());
    }

    LUAU_ASSERT!((*(*l).global).strt.nuse == 0);
  }
}

pub use lua_c_freeall as luaC_freeall;
