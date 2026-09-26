use core::ffi::c_void;

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{macros::setbvalue::setbvalue, type_aliases::t_value::TValue};

/// # Safety
/// `result` 须指向一块可写入 `TValue` 的有效内存（direct-access 回调交还宿主的 resultarg
/// 槽，本次调用期被栈钉住），且仅在 `LuauDirectFieldGet` 开启的 C ABI 直取路径中调用。
pub unsafe fn lua_userdatadirectfield_setboolean(result: *mut c_void, b: i32) {
  // Safety: 契约保证 `result` 指向可写 TValue 槽，setbvalue! 仅写 bool 值与 tag
  unsafe {
    LUAU_ASSERT!(fflag::LuauDirectFieldGet.get());
    setbvalue!(result as *mut TValue, b);
  }
}
