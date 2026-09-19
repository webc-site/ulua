use core::{
  ffi::{c_int, c_void},
  ptr::addr_of_mut,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_d_rawrunprotected_ldo::luaD_rawrunprotected,
  records::{call_context_lgc_alt_c::CallContext, lua_table::LuaTable},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn tableresizeprotected(l: *mut lua_State, t: *mut LuaTable, nhsize: c_int) {
  unsafe {
    let mut ctx = CallContext { t, nhsize };
    let status = luaD_rawrunprotected(l, Some(CallContext::run), addr_of_mut!(ctx) as *mut c_void);
    LUAU_ASSERT!(status == LuaStatus::Ok as c_int || status == LuaStatus::ErrMem as c_int);
  }
}
