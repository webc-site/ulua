use core::ffi::c_void;

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{macros::setnilvalue::setnilvalue, type_aliases::t_value::TValue};

/// # Safety
///
/// `result` 须指向一块可写入 `TValue` 的有效内存，且仅在
/// `LuauDirectFieldGet` 开启的 C ABI 直取路径中调用。
pub unsafe fn lua_userdatadirectfield_setnil(result: *mut c_void) {
  // Safety: 契约保证 `result` 指向可写 TValue 槽，仅写 nil 标签，不改其余字段
  unsafe {
    LUAU_ASSERT!(fflag::LuauDirectFieldGet.get());
    setnilvalue!(result as *mut TValue);
  }
}
