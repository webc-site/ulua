use crate::{
  enums::lua_type::LuaType, functions::index_2_addr::index_2_addr, macros::ttype::ttype,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 LuaState；`idx` 经 `index_2_addr` 解析为栈内合法 StkId（越界返回 `LUA_O_NILOBJECT` 亦可安全读取）。
/// 仅当槽为 function 且 `(*(*o).as_closure_ptr()).is_c == 0`（Lua 闭包）时返回 1，`clvalue` 解引用的闭包须存活。
/// 不抛错/不分配/不触碰 GC。cpp/VM/src/lapi.cpp:362 lua_isLfunction。
pub unsafe fn lua_is_lfunction(l: *mut LuaState, idx: i32) -> i32 {
  unsafe {
    let o: StkId = index_2_addr(l, idx);

    if ttype!(o) == LuaType::Function as u32 && (*(*o).as_closure_ptr()).is_c == 0 {
      1
    } else {
      0
    }
  }
}
