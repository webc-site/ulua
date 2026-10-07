use crate::{
  functions::murmur_hash_64b::murmur_hash_64b,
  macros::{gnode::gnode, lmod::lmod, sizenode::sizenode},
  records::{lua_node::LuaNode, lua_table::LuaTable},
};

/// 整数键哈希桶定位（w6e 入侧收形：`t: *const LuaTable → &LuaTable`，体内不再经
/// `(*t)` 重借用往返）。
///
/// 调用序契约（正确性，非内存安全——`t` 的存活前提已由 `&` 接收者类型承载）：表哈希
/// 部分须已分配——`node` 数组非空、`sizenode(t)` 为 2 的幂（供 `lmod` 取模），返回值
/// 恒落 `node[0..sizenode(t))` 内某槽。cpp `ltable.cpp:119`。
///
/// 出口形态判定（保留面，r12-w6d 逐点复核）：返回 `*mut LuaNode` 不收编——
/// `LuaNode` 挂在表 `d array` 柔性成员（布局绑定节点地址），本指针是
/// `mainposition`→`newkey` 的 `mp` 哨兵判据（`eq(mp, dummynode)`）与后续键/值槽写的
/// 可写 provenance 源头；构造点已收口为末句 `gnode!` 单表达式（宏自带 E1 裁决契约）。
pub(crate) fn hashint(t: &LuaTable, n: i64) -> *mut LuaNode {
  let bits = n as u64;
  #[cfg(target_endian = "little")]
  let (h1, h2) = (bits as u32, (bits >> 32) as u32);
  #[cfg(target_endian = "big")]
  let (h1, h2) = ((bits >> 32) as u32, bits as u32);

  // MurmurHash64B finalizer 单点见 murmur_hash_64b
  // ... truncated to 32-bit output (normally hash is equal to (uint64_t(h1) << 32) | h2, but we only really need the lower 32-bit half)
  let h2 = murmur_hash_64b(h1, h2);

  // We cast h2 to i32 to satisfy the lmod! macro's expectation of signed bitwise operands in the VM's specific lmod implementation.
  // SAFETY: 桶号经 `lmod!(…, sizenode!(t))` 恒落 `[0, sizenode(t))`，`node.add` 不越出
  // 表哈希数组（上方调用序契约；宏自带 E1 裁决口径的裸节点寻址保留面）。
  unsafe { gnode!(t, lmod!(h2 as i32, sizenode!(t)) as usize) }
}
