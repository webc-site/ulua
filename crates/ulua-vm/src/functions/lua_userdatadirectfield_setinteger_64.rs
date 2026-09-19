use core::ffi::c_void;

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{macros::setlvalue::setlvalue, type_aliases::t_value::TValue};

/// # Safety
///
/// `result` 须指向一块可写入 `TValue` 的有效内存，且仅在
/// `LuauDirectFieldGet` 开启的 C ABI 直取路径中调用。
#[cfg_attr(
  feature = "capi",
  unsafe(export_name = "ulua_lua_userdatadirectfield_setinteger64")
)]
pub unsafe fn lua_userdatadirectfield_setinteger64(result: *mut c_void, n: i64) {
  unsafe {
    LUAU_ASSERT!(fflag::LuauDirectFieldGet.get());
    setlvalue!(result as *mut TValue, n);
  }
}

/// # Safety
///
/// 同 [`lua_userdatadirectfield_setinteger64`]。
#[cfg_attr(
  feature = "capi",
  unsafe(export_name = "ulua_lua_userdatadirectfield_setinteger_64")
)]
pub unsafe fn lua_userdatadirectfield_setinteger_64(result: *mut c_void, n: i64) {
  unsafe {
    lua_userdatadirectfield_setinteger64(result, n);
  }
}
