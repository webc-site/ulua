use core::{ffi::c_void, ptr::addr_of_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_d_rawrunprotected_ldo::lua_d_rawrunprotected,
  records::{call_context_lgc::StringResizeCallContext as CallContext, lua_state::LuaState},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn stringresizeprotected(l: *mut LuaState, newsize: i32) {
  unsafe {
    let mut ctx = CallContext { newsize };
    let status = lua_d_rawrunprotected(l, Some(CallContext::run), addr_of_mut!(ctx) as *mut c_void);
    LUAU_ASSERT!(status == LuaStatus::Ok as i32 || status == LuaStatus::ErrMem as i32);
  }
}
