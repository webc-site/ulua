use core::ptr::null;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_throw_ldo::lua_d_throw, lua_g_runerror_l::lua_g_runerror_l},
  macros::luai_maxccalls::LUAI_MAXCCALLS,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState`，读其 `(*l).n_ccalls` 计数；达 `LUAI_MAXCCALLS` 会 `lua_g_runerror` 抛
/// "C stack overflow"、达硬上限会 `lua_d_throw(ErrErr)`——两者均以 longjmp/unwind 逃逸，故须在受保护帧内调用。
/// cpp `ldo.cpp:244`。
pub unsafe fn lua_d_check_cstack(l: *mut LuaState) {
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
