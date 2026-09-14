use core::ffi::{c_int, c_void};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_d_rawrunprotected_ldo::luaD_rawrunprotected,
  records::{call_context_lgc_alt_c::CallContext, lua_table::LuaTable},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn tableresizeprotected(l: *mut lua_State, t: *mut LuaTable, nhsize: c_int) {
  unsafe {
    let mut ctx = CallContext { t, nhsize };
    let status = luaD_rawrunprotected(
      l,
      Some(CallContext::run),
      core::ptr::addr_of_mut!(ctx) as *mut c_void,
    );
    LUAU_ASSERT!(status == LuaStatus::Ok as c_int || status == LuaStatus::ErrMem as c_int);
  }
}
