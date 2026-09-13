use core::ffi::c_void;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{macros::setnvalue::setnvalue, type_aliases::t_value::TValue};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_userdatadirectfield_setnumber")]
pub unsafe fn lua_userdatadirectfield_setnumber(result: *mut c_void, n: f64) {
  unsafe {
    LUAU_ASSERT!(FFlag::LuauDirectFieldGet.get());
    setnvalue!(result as *mut TValue, n);
  }
}
