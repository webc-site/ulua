use crate::{
  functions::index_2_addr::index_2_addr,
  macros::{api_check::api_check, lua_utag_limit::LUA_UTAG_LIMIT},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// `lua_setuserdatatag`（cpp/VM/src/lapi.cpp:1917）：把栈位 `idx` 处 full userdata 的
/// tag 覆写为 `tag`。调用序契约（正确性，非内存安全；r16-v4 引用形前移取
/// `&mut LuaState`——实际写点 `u.tag = …` 落在经 `*l` 栈槽可达的 GC 堆对象上，属调用方
/// 可见的堆变更，须由独占借用承载，并存 `&LuaState` 下裸写会破 Rust 别名面）：
/// `idx` 经 `index_2_addr` 解析（该步只读 `l`，独占引用短借即还）为指向 full userdata
/// 的栈槽（`api_check` 断言 `is_userdata`，release 非 userdata 由写点 `as_udata_mut`
/// 返回 `None` 静默跳过，cpp 同形）；`tag < LUA_UTAG_LIMIT`（`api_check` 断言）。
/// 不抛错/不分配；unsafe 内移到真实裸触点（StkId 解引用链）。
pub fn lua_setuserdatatag(l: &mut LuaState, idx: i32, tag: i32) {
  api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
  let o: StkId = index_2_addr(l, idx);
  // SAFETY: 契约保证 `l` 存活（类型承载）、`idx` 解析出的栈槽在使用点有效（栈未重
  // 分配）、其为 full userdata 且所指 Udata 存活可写、`tag` 界内；块内仅 StkId 裸解
  // 引用读数与 Udata.tag 落笔，无重入/分配穿插。
  unsafe {
    api_check!(l, (*o).is_userdata());
    if let Some(u) = (*(*o).value.gc).as_udata_mut() {
      u.tag = tag as u8;
    }
  }
}
