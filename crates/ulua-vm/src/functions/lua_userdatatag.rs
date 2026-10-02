use crate::{
  functions::index_2_addr::index_2_addr, records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// `lua_userdatatag`（cpp/VM/src/lapi.cpp 同名）：读栈位 `idx` 处 userdata 的 tag，
/// 非 userdata 得 `-1`。r16-v4 引用形前移取 `&LuaState`（纯读数，槽址解析与 tag 读取
/// 均不改 VM）：`idx` 由 `index_2_addr` 只读解析，返回槽地址在读点前有效（栈未重分配
/// 契约同 `index_2_addr` 文档）；越界/哨兵槽读数收敛为确定结果，无栈外指针算术。
/// unsafe 内移到真实裸触点（StkId 解引用读数）。
pub fn lua_userdatatag(l: &LuaState, idx: i32) -> i32 {
  let o: StkId = index_2_addr(l, idx);
  // SAFETY: 契约保证 `l` 存活（类型承载）、`idx` 解析出的槽地址（含 LUA_O_NILOBJECT
  // 哨兵）在使用点可读；块内仅该槽与其 userdata 载荷的只读解引用，无写点无重入。
  unsafe {
    if (*o).is_userdata() {
      (*o).as_userdata().tag as i32
    } else {
      -1
    }
  }
}
