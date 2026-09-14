use core::ffi::c_int;

use crate::{macros::lua_callinfo_native::LUA_CALLINFO_NATIVE, records::lua_state::lua_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaG_isnative")]
pub unsafe fn lua_g_isnative(l: *mut lua_State, level: c_int) -> c_int {
  unsafe {
    if (level as u32) >= ((*l).ci.offset_from((*l).base_ci) as u32) {
      return 0;
    }

    let ci = (*l).ci.offset(-level as isize);
    if ((*ci).flags & LUA_CALLINFO_NATIVE as u32) != 0 {
      1
    } else {
      0
    }
  }
}

pub use lua_g_isnative as luaG_isnative;
