use core::ffi::c_void;

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{macros::setlvalue::setlvalue, type_aliases::t_value::TValue};

/// # Safety
///
/// `result` 须指向一块可写入 `TValue` 的有效内存，且仅在
/// `LuauDirectFieldGet` 开启的 C ABI 直取路径中调用。
pub unsafe fn lua_userdatadirectfield_setinteger64(result: *mut c_void, n: i64) {
  // Safety: 契约保证 `ud`/`result` 指向存活 userdata 的可写字段区且字段偏移在注册描述符界内（直取路径仅在 LuauDirectFieldGet 开启时调用）
  unsafe {
    LUAU_ASSERT!(fflag::LuauDirectFieldGet.get());
    setlvalue!(result as *mut TValue, n);
  }
}

/// # Safety
///
/// 同 [`lua_userdatadirectfield_setinteger64`]。
pub unsafe fn lua_userdatadirectfield_setinteger_64(result: *mut c_void, n: i64) {
  // Safety: 契约保证 `ud`/`result` 指向存活 userdata 的可写字段区且字段偏移在注册描述符界内（直取路径仅在 LuauDirectFieldGet 开启时调用）
  unsafe {
    lua_userdatadirectfield_setinteger64(result, n);
  }
}
