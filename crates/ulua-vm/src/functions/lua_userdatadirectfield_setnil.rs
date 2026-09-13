use core::ffi::c_void;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{macros::setnilvalue::setnilvalue, type_aliases::t_value::TValue};

#[unsafe(export_name = "ulua_lua_userdatadirectfield_setnil")]
pub(crate) unsafe fn lua_userdatadirectfield_setnil(result: *mut c_void) {
  unsafe {
    LUAU_ASSERT!(FFlag::LuauDirectFieldGet.get());
    setnilvalue!(result as *mut TValue);
  }
}
