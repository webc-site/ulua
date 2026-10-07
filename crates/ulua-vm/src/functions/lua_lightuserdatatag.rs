use crate::{
  functions::index_2_addr::index_2_addr, macros::lightuserdatatag::lightuserdatatag,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// `lua_lightuserdatatag`（cpp/VM/src/lapi.cpp 同名）：读栈位 `idx` 处 lightuserdata
/// 的 tag，非 lightuserdata 得 `-1`。r16-v4 引用形前移取 `&LuaState`（纯读数，不改
/// VM）：`idx` 由 `index_2_addr` 只读解析，返回槽地址在读点前有效（栈未重分配契约同
/// `index_2_addr` 文档）。unsafe 内移到真实裸触点（StkId 解引用读数与
/// `lightuserdatatag!` 展开的 `(*o)` 读取）。
pub fn lua_lightuserdatatag(l: &LuaState, idx: i32) -> i32 {
  let o: StkId = index_2_addr(l, idx);
  // SAFETY: 契约保证 `l` 存活（类型承载）、`idx` 解析出的槽地址（含 LUA_O_NILOBJECT
  // 哨兵）在使用点可读；块内仅该槽 lightuserdata 载荷的只读解引用，无写点无重入。
  unsafe {
    if (*o).is_lightuserdata() {
      lightuserdatatag!(o)
    } else {
      -1
    }
  }
}
