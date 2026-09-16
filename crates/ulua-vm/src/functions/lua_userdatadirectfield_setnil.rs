use core::ffi::c_void;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{macros::setnilvalue::setnilvalue, type_aliases::t_value::TValue};

/// # Safety
///
/// `result` 须指向一块可写入 `TValue` 的有效内存，且仅在
/// `LuauDirectFieldGet` 开启的 C ABI 直取路径中调用。
#[cfg_attr(
  feature = "capi",
  unsafe(export_name = "ulua_lua_userdatadirectfield_setnil")
)]
pub unsafe fn lua_userdatadirectfield_setnil(result: *mut c_void) {
  unsafe {
    LUAU_ASSERT!(FFlag::LuauDirectFieldGet.get());
    setnilvalue!(result as *mut TValue);
  }
}
