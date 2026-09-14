use core::ffi::c_void;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{macros::setvvalue::setvvalue, type_aliases::t_value::TValue};

#[unsafe(export_name = "ulua_lua_userdatadirectfield_setvector_void_f32_f32_f32_f32")]
pub(crate) unsafe fn lua_userdatadirectfield_setvector_void_f32_f32_f32_f32(
  result: *mut c_void,
  x: f32,
  y: f32,
  z: f32,
  w: f32,
) {
  unsafe {
    LUAU_ASSERT!(FFlag::LuauDirectFieldGet.get());
    setvvalue!(result as *mut TValue, x, y, z, w);
  }
}
