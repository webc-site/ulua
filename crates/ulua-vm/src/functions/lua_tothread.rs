use crate::{
  enums::value_view::ValueView, functions::index_2_addr::index_2_addr,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// cpp `lua_tothread`（`VM/src/lapi.cpp:644`）：`idx` 槽为 thread 时返回其协程 `LuaState`
/// 指针，否则 `None`。
///
/// 空指针哨兵收口为 `Option<*mut LuaState>`。载荷刻意保持裸指针而非 `&'a mut
/// LuaState`：`LuaState` 在执行中被 VM 持续原地改写，给出引用会谎称独占/非别名（Stacked
/// Borrows 违例），[`ValueView::Thread`] 本身即以裸指针承载该 payload。`None` 即"非线程槽"。
///
/// # Safety
/// `l` 须为存活 `LuaState` 且 `idx` 为合法（伪）索引；命中的 thread 值其 `(*o).value.gc` 须指向
/// 存活 thread GCObject（返回其内嵌 `th` 状态指针）。
pub unsafe fn lua_tothread(l: *mut LuaState, idx: i32) -> Option<*mut LuaState> {
  unsafe {
    let o: StkId = index_2_addr(l, idx);

    match ValueView::from_tvalue(&*o) {
      ValueView::Thread(th) => Some(th),
      _ => None,
    }
  }
}
