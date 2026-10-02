use crate::{
  enums::lua_type::LuaType, functions::index_2_addr::index_2_addr, macros::ttype::ttype,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// `lua_is_lfunction`（cpp/VM/src/lapi.cpp:362 lua_isLfunction）。r16-v4b 引用形前移
/// 取 `&LuaState`（纯 tt 读数，不改 VM；`l` 存活由类型承载），unsafe fn 消亡转 safe
/// fn，unsafe 内移到真实裸触点（StkId 解引用与 `clvalue` 闭包指针读数）。
/// 调用序契约（正确性，非内存安全）：`idx` 经 `index_2_addr` 解析为栈内合法 StkId
/// （越界返回 `LUA_O_NILOBJECT` 亦可安全读取），所得槽在使用点可读（栈未重分配）；
/// 仅当槽为 function 且 `(*(*o).as_closure_ptr()).is_c == 0`（Lua 闭包）时返回 1，
/// 命中时 `clvalue` 解引用前提=该槽 tag 与 payload 配套（VM tag 不变量）。
/// 不抛错/不分配/不触碰 GC。
pub fn lua_is_lfunction(l: &LuaState, idx: i32) -> i32 {
  let o: StkId = index_2_addr(l, idx);

  // SAFETY: 契约保证 `l` 存活（类型承载）、`idx` 解析出的槽（含 `LUA_O_NILOBJECT`
  // 哨兵）在使用点可读；块内仅该槽 tag 读数与命中 Function 后的 is_c 位只读，
  // 无写点无重入。
  unsafe {
    if ttype!(o) == LuaType::Function as u32 && (*(*o).as_closure_ptr()).is_c == 0 {
      1
    } else {
      0
    }
  }
}
