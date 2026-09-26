use core::ptr::copy_nonoverlapping;

use crate::{
  functions::murmur_hash_64b::murmur_hash_64b,
  macros::{gnode::gnode, lmod::lmod, sizenode::sizenode},
  records::{lua_node::LuaNode, lua_table::LuaTable},
};

/// # Safety
/// `t` 须为存活 `LuaTable` 且哈希部分已分配：`gnode!(t, i)`/`sizenode!(t)` 依赖 `node` 数组非空、
/// `sizenode(t)` 为 2 的幂（供 `lmod` 取模），返回指向 `node[0..sizenode(t)]` 内某槽的可写指针。
/// cpp `ltable.cpp:119`。
pub(crate) unsafe fn hashint(t: *const LuaTable, n: i64) -> *mut LuaNode {
  unsafe {
    // static_assert(sizeof(n) == sizeof(unsigned int) * 2, "expected a 8-byte integer");
    let mut i: [u32; 2] = [0; 2];
    copy_nonoverlapping(&n as *const i64 as *const u8, i.as_mut_ptr() as *mut u8, 8);

    // MurmurHash64B finalizer 单点见 murmur_hash_64b
    // ... truncated to 32-bit output (normally hash is equal to (uint64_t(h1) << 32) | h2, but we only really need the lower 32-bit half)
    let h2 = murmur_hash_64b(i[0], i[1]);

    // We cast h2 to i32 to satisfy the lmod! macro's expectation of signed bitwise operands in the VM's specific lmod implementation.
    gnode!(t, lmod!(h2 as i32, sizenode!(t)) as usize)
  }
}
