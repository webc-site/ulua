//! Source: `VM/src/ldebug.cpp:256-262` (hand-ported)

use crate::{
  functions::{cstr_cow, lua_t_objtypename::lua_t_objtypename},
  macros::lua_g_runerror::lua_g_runerror,
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState`；`p1`/`p2` 须为指向栈上参与拼接的两个操作数 TValue 的有效指针
/// （`luaT_objtypename` 读其类型），`luaG_runerror` 抛错且永不返回，须在受保护帧内调用。cpp `ldebug.cpp:290`。
pub(crate) unsafe fn lua_g_concaterror(l: *mut LuaState, p1: StkId, p2: StkId) -> ! {
  // SAFETY: 契约保证 `l`/栈槽存活可读；`lua_t_objtypename` 返回的 C 串指针立即经
  // `cstr_cow` 收口为 `Cow<str>`（unsafe 关在门面内），本函数不再出现宿主 C 串裸指针。
  unsafe {
    let t1 = cstr_cow(lua_t_objtypename(&*l, &*p1));
    let t2 = cstr_cow(lua_t_objtypename(&*l, &*p2));

    lua_g_runerror!(l, "attempt to concatenate {} with {}", t1, t2)
  }
}
