use crate::{
  functions::index_2_addr::index_2_addr, records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 LuaState；`idx` 经 `index_2_addr` 解析为栈内合法 StkId（越界返回可读的 `LUA_O_NILOBJECT`）。
/// 仅按 tt 判 full/light userdata，不解引用 payload。不抛错/不分配/不触碰 GC。cpp/VM/src/lapi.cpp:381 lua_isuserdata。
pub unsafe fn lua_isuserdata(l: *mut LuaState, idx: i32) -> i32 {
  unsafe {
    let o: *const TValue = index_2_addr(l, idx);
    ((*o).is_userdata() || (*o).is_lightuserdata()) as i32
  }
}
