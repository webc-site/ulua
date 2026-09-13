use core::ptr::null;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_d_realloc_ci::lua_d_realloc_ci, lua_d_throw_ldo::lua_d_throw,
    lua_g_runerror_l::lua_g_runerror_l,
  },
  macros::luai_maxcalls::LUAI_MAXCALLS,
  type_aliases::{call_info::CallInfo, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaD_growCI")]
pub unsafe fn lua_d_grow_ci(l: *mut lua_State) -> *mut CallInfo {
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

pub use lua_d_grow_ci as luaD_growCI;
