/// C++ `#define gnode(t,i) (&(t)->node[i])`：哈希桶 `i` 的裸 `*mut LuaNode`。
///
/// 保持裸指针形（r12-E1 裁决）：
/// - 指针身份被写路径依赖——`luaH_newkey` 在哨兵表上经 `mainposition`→本宏取到
///   哨兵基址后用指针相等判据 `eq(mp, dummynode)` 触发 rehash（newkey.rs），
///   `findindex` 用 `gnode!(t, 0)` 作 `offset_from` 基址；哨兵桶不可出借 `&mut`，
///   切片化要么破坏哨兵分支要么是同址往返，unsafe 总量不减。
/// - 越界索引 `i >= sizenode(t)` 时本宏是 UB，与 cpp 宏口径逐位一致。
#[macro_export]
macro_rules! gnode {
  ($t:expr, $i:expr) => {
    (*$t).node.add($i as usize)
  };
}

pub use gnode;
