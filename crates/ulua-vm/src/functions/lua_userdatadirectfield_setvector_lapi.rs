use core::ffi::c_void;

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{macros::setvvalue::setvvalue, type_aliases::t_value::TValue};

/// # Safety
///
/// `result` 须指向一块可写入 `TValue` 的有效内存，且仅在
/// `LuauDirectFieldGet` 开启的 C ABI 直取路径中调用。
pub unsafe fn lua_userdatadirectfield_setvector_void_f32_f32_f32_f32(
  result: *mut c_void,
  x: f32,
  y: f32,
  z: f32,
  w: f32,
) {
  // Safety: 契约保证 `ud`/`result` 指向存活 userdata 的可写字段区且字段偏移在注册描述符界内（直取路径仅在 LuauDirectFieldGet 开启时调用）
  unsafe {
    LUAU_ASSERT!(fflag::LuauDirectFieldGet.get());
    setvvalue!(result as *mut TValue, x, y, z, w);
  }
}

/// # Safety
///
/// `result` 须指向一块可写入 `TValue` 的有效内存，且仅在
/// `LuauDirectFieldGet` 开启的 C ABI 直取路径中调用。
pub unsafe fn lua_userdatadirectfield_setvector_void_f32_f32_f32(
  result: *mut c_void,
  x: f32,
  y: f32,
  z: f32,
) {
  // Safety: 契约保证 `ud`/`result` 指向存活 userdata 的可写字段区且字段偏移在注册描述符界内（直取路径仅在 LuauDirectFieldGet 开启时调用）
  unsafe {
    LUAU_ASSERT!(fflag::LuauDirectFieldGet.get());
    setvvalue!(result as *mut TValue, x, y, z, 0.0f32);
  }
}
