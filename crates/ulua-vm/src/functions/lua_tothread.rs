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
/// r16-v8 收口：`pub unsafe fn` → `pub fn`，首参转引用形（非空/对齐由引用承载，
/// v5 `lua_o_rawequal_obj` 同款话术）。r19-w2 再收窄为 `&LuaState`：本体只经
/// [`index_2_addr`] 读一个槽并取 payload 裸指针，从不写 `l`；原先的 `&mut` 是过度授权，
/// 迫使 `&self` 侧的调用方（`LuaState::to_thread`）从共享引用变造独占借用。
/// # Safety
/// 调用序契约（正确性，非内存安全；safe fn 文档断言，由调用方承载）：`idx` 为合法（伪）索引；
/// 命中的 thread 值其 `(*o).value.gc` 须指向存活 thread GCObject（返回其内嵌 `th` 状态指针）。
pub fn lua_tothread(l: &LuaState, idx: i32) -> Option<*mut LuaState> {
  // SAFETY: `o` 为 `index_2_addr` 按契约给出的栈槽地址，此处只读其 TValue 头取
  // payload 裸指针，从不解引用该指针本身。
  unsafe {
    let o: StkId = index_2_addr(l, idx);

    match ValueView::from_tvalue(&*o) {
      ValueView::Thread(th) => Some(th),
      _ => None,
    }
  }
}
