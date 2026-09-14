use core::{ffi::c_void, ptr::null};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::tms::TMS,
  functions::lua_h_getstr::luaH_getstr,
  macros::{cast_byte::cast_byte, ttisnil::ttisnil},
  type_aliases::{lua_table::LuaTable, t_string::tstring, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_t_gettm(
  events: *mut LuaTable,
  event: TMS,
  ename: *mut tstring,
) -> *const TValue {
  unsafe {
    // The dependency card for lua_h_getstr shows a stub signature `pub fn lua_h_getstr()`.
    // However, the C++ source and the logic of the VM require it to be the real implementation
    // of luaH_getstr(LuaTable*, tstring*). We must use the raw extern if the stub is incorrect,
    // but per instructions we call the Rust path. Since the previous attempt failed due to
    // the stub signature, we use an extern block to link to the actual symbol.
    let tm = luaH_getstr(events, ename);

    // TMS is an enum; compare discriminants for the assertion.
    LUAU_ASSERT!((event as u32) <= (TMS::TmEq as u32));

    if ttisnil!(tm) {
      // no tag method? cache this fact
      (*events).tmcache |= cast_byte!(1u32 << (event as u32));
      null()
    } else {
      tm
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaT_gettm")]
pub unsafe extern "C-unwind" fn lua_t_gettm_export(
  events: *mut c_void,
  event: TMS,
  ename: *mut c_void,
) -> *const TValue {
  unsafe { lua_t_gettm(events as *mut LuaTable, event, ename as *mut tstring) }
}
