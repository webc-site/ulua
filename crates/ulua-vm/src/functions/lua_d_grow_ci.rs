use core::ptr::null;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_d_realloc_ci::lua_d_realloc_ci, lua_d_throw_ldo::lua_d_throw,
    lua_g_runerror_l::lua_g_runerror_l,
  },
  macros::luai_maxcalls::LUAI_MAXCALLS,
  records::{call_info::CallInfo, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活 `LuaState`：读改 `(*l).size_ci/ci`，`lua_d_realloc_ci` 重排 `base_ci` 后旧 `ci` 指针失效，
/// 故返回 `(*l).ci.add(1)` 时须保证 `(*l).ci` 落在新 `base_ci[0..size_ci]` 区间；超 `LUAI_MAXCALLS`/hardlimit 经
/// `lua_d_throw`/`lua_g_runerror_l` 抛栈溢出错误——调用方须处于受保护帧承接展开，可触发 GC。
/// cpp VM/src/ldo.cpp:227
pub unsafe fn lua_d_grow_ci(l: *mut LuaState) -> *mut CallInfo {
  unsafe {
    // allow extra stack space to handle stack overflow in xpcall
    let hardlimit: i32 = LUAI_MAXCALLS + (LUAI_MAXCALLS >> 3);

    if (*l).size_ci >= hardlimit {
      // error while handling stack error
      lua_d_throw(l, LuaStatus::ErrErr as i32);
    }

    let request: i32 = (*l).size_ci * 2;
    let new_size = if (*l).size_ci >= LUAI_MAXCALLS {
      hardlimit
    } else if request < LUAI_MAXCALLS {
      request
    } else {
      LUAI_MAXCALLS
    };

    lua_d_realloc_ci(l, new_size);

    if (*l).size_ci > LUAI_MAXCALLS {
      lua_g_runerror_l(l, null(), format_args!("stack overflow"));
    }

    (*l).ci = (*l).ci.add(1);
    (*l).ci
  }
}
