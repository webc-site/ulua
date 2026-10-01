use crate::{
  functions::murmur_hash_64b::murmur_hash_64b,
  macros::{gnode::gnode, lmod::lmod, sizenode::sizenode},
  records::{lua_node::LuaNode as LuaNodeAlias, lua_table::LuaTable as LuaTableAlias},
};

/// # Safety
/// `t` 须为存活 `LuaTable` 的共享只读借用，其 `node` 指针非空且哈希数组长度恰为 `(*t).sizenode`（由 rehash/新建保证）；
/// 返回的 `gnode!(t, lmod!(h2, sizenode))` 落在 `[node, node+sizenode)` 区间内，仅在 sizenode>0 时有效
/// （sizenode==0 时由调用方走数组分支，不入本函数）。`n` 为已判别的数字键值，无副作用、不分配、不抛错。
/// cpp VM/src/ltable.cpp:90
pub(crate) unsafe fn hashnum(t: &LuaTableAlias, n: f64) -> *mut LuaNodeAlias {
  unsafe {
    // static_assert(sizeof(double) == sizeof(unsigned int) * 2, "expected a 8-byte double");
    let bits = n.to_bits();
    #[cfg(target_endian = "little")]
    let (h1, h2) = (bits as u32, ((bits >> 32) as u32) & 0x7fffffff);
    #[cfg(target_endian = "big")]
    let (h1, h2) = (((bits >> 32) as u32), (bits as u32) & 0x7fffffff);

    // MurmurHash64B finalizer 单点见 murmur_hash_64b
    // ... truncated to 32-bit output (normally hash is equal to (uint64_t(h1) << 32) | h2, but we only really need the lower 32-bit half)
    let h2 = murmur_hash_64b(h1, h2);

    gnode!(t, lmod!(h2 as i32, sizenode!(t)) as usize)
  }
}
