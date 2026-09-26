//! Source: `VM/src/ldebug.cpp:256-262` (hand-ported)

use core::ffi::c_char;

use crate::{
  functions::{cstr_cow, lua_t_objtypename::lua_t_objtypename},
  macros::lua_g_runerror::lua_g_runerror,
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState`；`p1`/`p2` 须为指向栈上参与拼接的两个操作数 TValue 的有效指针
/// （`luaT_objtypename` 读其类型），`luaG_runerror` 抛错且永不返回，须在受保护帧内调用。cpp `ldebug.cpp:290`。
pub unsafe fn lua_g_concaterror(l: *mut LuaState, p1: StkId, p2: StkId) -> ! {
  unsafe {
    let t1: *const c_char = lua_t_objtypename(l, p1);
    let t2: *const c_char = lua_t_objtypename(l, p2);

    lua_g_runerror!(
      l,
      "attempt to concatenate {} with {}",
      cstr_cow(t1),
      cstr_cow(t2)
    )
  }
}
