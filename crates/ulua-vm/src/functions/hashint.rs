use crate::{
  functions::murmur_hash_64b::murmur_hash_64b,
  macros::{gnode::gnode, lmod::lmod, sizenode::sizenode},
  records::{lua_node::LuaNode, lua_table::LuaTable},
};

/// # Safety
/// `t` 须为存活 `LuaTable` 且哈希部分已分配：`gnode!(t, i)`/`sizenode!(t)` 依赖 `node` 数组非空、
/// `sizenode(t)` 为 2 的幂（供 `lmod` 取模），返回指向 `node[0..sizenode(t)]` 内某槽的可写指针。
/// cpp `ltable.cpp:119`。
///
/// r12-w6d 逐点复核定性（保留面）：本点位是 `mainposition`→`newkey` 的 `mp` 哨兵指针
/// 判据（`eq(mp, dummynode)`）与后续键/值槽写的指针源头，返回值须保持挂表裸指针下的
/// 可写 provenance（mainposition 契约明载不得降为 `&LuaTable`），非纯下标读写，不收编。
pub(crate) unsafe fn hashint(t: *const LuaTable, n: i64) -> *mut LuaNode {
  unsafe {
    let bits = n as u64;
    #[cfg(target_endian = "little")]
    let (h1, h2) = (bits as u32, (bits >> 32) as u32);
    #[cfg(target_endian = "big")]
    let (h1, h2) = ((bits >> 32) as u32, bits as u32);

    // MurmurHash64B finalizer 单点见 murmur_hash_64b
    // ... truncated to 32-bit output (normally hash is equal to (uint64_t(h1) << 32) | h2, but we only really need the lower 32-bit half)
    let h2 = murmur_hash_64b(h1, h2);

    // We cast h2 to i32 to satisfy the lmod! macro's expectation of signed bitwise operands in the VM's specific lmod implementation.
    gnode!(t, lmod!(h2 as i32, sizenode!(t)) as usize)
  }
}
