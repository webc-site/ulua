/// C++ `#define gnode(t,i) (&(t)->node[i])`：哈希桶 `i` 的裸 `*mut LuaNode`。
///
/// 保持裸指针形（r12-E1 裁决，不收编进 [`LuaTable::node_window`] 切片访问器）：
/// - 指针身份被写路径依赖——`luaH_newkey` 在哨兵表上经 `mainposition`→本宏取到
///   哨兵基址后用指针相等判据 `eq(mp, dummynode)` 触发 rehash（newkey.rs），
///   `findindex` 用 `gnode!(t, 0)` 作 `offset_from` 基址；哨兵桶不可出借 `&mut`，
///   切片化要么破坏哨兵分支要么是同址往返，unsafe 总量不减。
/// - 与窗访问器的逐位一致契约（有效态下）：`gnode!(t, i)` ⇔ `t.node_window()[i]`
///   的地址，`i ∈ 0..sizenode(t)`；`i` 越界本宏仍是 UB，切片窗仍是 panic。
///   消费方向 E2–E4 迁移时优先改走 `node_window`/`node_window_mut` 索引，
///   仅指针算术不可省处保留本宏。
///
/// [`LuaTable::node_window`]: crate::records::lua_table::LuaTable::node_window
#[macro_export]
macro_rules! gnode {
  ($t:expr, $i:expr) => {
    (*$t).node.add($i as usize)
  };
}

pub use gnode;
