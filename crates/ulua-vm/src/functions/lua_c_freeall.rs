use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{deletegco::deletegco, lua_m_visitgco::lua_m_visitgco},
  records::{global_state::global_State, lua_state::LuaState},
};

/// # Safety
/// `l` 须为其所属 `global` 状态的主线程（`LUAU_ASSERT l==(*g).mainthread`，取 `(*l).global`）；
/// 调用后 `allgc` 全链、字符串表 `strt.hash[0..size]` 均被 `deletegco` 逐个释放，故不得再有存活对象引用，
/// 且须在无任何 GC/分配在跑时调用。cpp `lgc.cpp:857`。
pub unsafe fn lua_c_freeall(l: *mut LuaState) {
  unsafe {
    let g: *mut global_State = (*l).global;

    LUAU_ASSERT!(l == (*g).mainthread);

    lua_m_visitgco(l, l as *mut c_void, deletegco);

    // free 后全部 bucket 应为空
    for &bucket in (*g).strt.buckets() {
      LUAU_ASSERT!(bucket.is_null());
    }

    LUAU_ASSERT!((*(*l).global).strt.nuse == 0);
  }
}
