use core::ffi::c_void;

use crate::{
  functions::lua_d_callny::lua_d_callny,
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
///
/// `l` and `ud` must be valid pointers.
#[unsafe(export_name = "ulua_luaB_xpcallerr")]
pub unsafe fn lua_b_xpcallerr(l: *mut lua_State, ud: *mut c_void) {
  unsafe {
    let func: StkId = ud as StkId;
    lua_d_callny(l, func, 1);
  }
}

pub use lua_b_xpcallerr as luaB_xpcallerr;
