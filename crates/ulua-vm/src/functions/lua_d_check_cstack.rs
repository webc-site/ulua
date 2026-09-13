use core::ptr::null;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_throw_ldo::lua_d_throw, lua_g_runerror_l::lua_g_runerror_l},
  macros::luai_maxccalls::LUAI_MAXCCALLS,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaD_checkCstack")]
pub unsafe fn lua_d_check_cstack(l: *mut lua_State) {
  unsafe {
    // allow extra stack space to handle stack overflow in xpcall
    let hardlimit: i32 = LUAI_MAXCCALLS + (LUAI_MAXCCALLS >> 3);

    if (*l).n_ccalls as i32 == LUAI_MAXCCALLS {
      lua_g_runerror_l(l, null(), format_args!("C stack overflow"));
    } else if (*l).n_ccalls as i32 >= hardlimit {
      lua_d_throw(l, LuaStatus::ErrErr as i32);
    }
  }
}

pub use lua_d_check_cstack as luaD_checkCstack;
