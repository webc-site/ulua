use core::ffi::c_void;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{macros::setlvalue::setlvalue, type_aliases::t_value::TValue};

#[unsafe(export_name = "ulua_lua_userdatadirectfield_setinteger64")]
pub(crate) unsafe fn lua_userdatadirectfield_setinteger64(result: *mut c_void, n: i64) {
  unsafe {
    LUAU_ASSERT!(FFlag::LuauDirectFieldGet.get());
    setlvalue!(result as *mut TValue, n);
  }
}

#[unsafe(export_name = "ulua_lua_userdatadirectfield_setinteger_64")]
pub(crate) unsafe fn lua_userdatadirectfield_setinteger_64(result: *mut c_void, n: i64) {
  unsafe {
    lua_userdatadirectfield_setinteger64(result, n);
  }
}
