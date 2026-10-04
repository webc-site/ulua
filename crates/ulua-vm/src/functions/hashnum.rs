use crate::{
  functions::murmur_hash_64b::murmur_hash_64b,
  macros::{gnode::gnode, lmod::lmod, sizenode::sizenode},
  records::{lua_node::LuaNode, lua_table::LuaTable},
};

/// 数字键哈希桶定位（w6e 诚实降级：入侧本已是 `&LuaTableAlias` 引用形，体内唯
/// 一表达式 `gnode!`/`sizenode!` 对引用解引用皆安全，故去 `unsafe fn` 屏障）。
///
/// 调用序契约（正确性，非内存安全——`t` 的存活已由 `&` 承载）：其 `node` 指针非空
/// 且哈希数组长度恰为 `(*t).lsizenode` 折算的 2 的幂（由 rehash/新建保证）；仅在
/// sizenode>0 时有效（sizenode==0 时由调用方走数组分支，不入本函数）。`n` 为已判别
/// 的数字键值，无副作用、不分配、不抛错。cpp VM/src/ltable.cpp:90。
///
/// 出口形态判定（保留面，r12-w6d 逐点复核）：返回 `*mut LuaNode` 不收编——
/// `LuaNode` 属表 `d array` 柔性成员的布局绑定节点地址，本指针是 `mp` 哨兵判据
/// （`eq(mp, dummynode)`）的裸指针源头；构造点收口为末句 `gnode!` 单表达式。
pub(crate) fn hashnum(t: &LuaTable, n: f64) -> *mut LuaNode {
  // static_assert(sizeof(double) == sizeof(unsigned int) * 2, "expected a 8-byte double");
  let bits = n.to_bits();
  #[cfg(target_endian = "little")]
  let (h1, h2) = (bits as u32, ((bits >> 32) as u32) & 0x7fffffff);
  #[cfg(target_endian = "big")]
  let (h1, h2) = (((bits >> 32) as u32), (bits as u32) & 0x7fffffff);

  // MurmurHash64B finalizer 单点见 murmur_hash_64b
  // ... truncated to 32-bit output (normally hash is equal to (uint64_t(h1) << 32) | h2, but we only really need the lower 32-bit half)
  let h2 = murmur_hash_64b(h1, h2);

  // SAFETY: 桶号经 `lmod!(…, sizenode!(t))` 恒落 `[0, sizenode(t))`，`node.add` 不越出
  // 表哈希数组（上方调用序契约；宏自带 E1 裁决口径的裸节点寻址保留面）。
  unsafe { gnode!(t, lmod!(h2 as i32, sizenode!(t)) as usize) }
}
