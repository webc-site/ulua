use crate::{
  functions::murmur_hash_64b::murmur_hash_64b,
  macros::{gnode::gnode, lmod::lmod, sizenode::sizenode},
  records::{lua_node::LuaNode as LuaNodeAlias, lua_table::LuaTable as LuaTableAlias},
};

/// # Safety
/// `t` 须为存活 `LuaTable`，其 `node` 指针非空且哈希数组长度恰为 `(*t).sizenode`（由 rehash/新建保证）；
/// 返回的 `gnode!(t, lmod!(h2, sizenode))` 落在 `[node, node+sizenode)` 区间内，仅在 sizenode>0 时有效
/// （sizenode==0 时由调用方走数组分支，不入本函数）。`n` 为已判别的数字键值，无副作用、不分配、不抛错。
/// cpp VM/src/ltable.cpp:90
pub(crate) unsafe fn hashnum(t: *mut LuaTableAlias, n: f64) -> *mut LuaNodeAlias {
  unsafe {
    // static_assert(sizeof(double) == sizeof(unsigned int) * 2, "expected a 8-byte double");
    // to_bits 即语言保证的位重解释，替代 C 的 memcpy 双字别名访问（本机字节序）
    let [b0, b1, b2, b3, b4, b5, b6, b7] = n.to_bits().to_ne_bytes();
    // mask out sign bit to make sure -0 and 0 hash to the same value
    let h1 = u32::from_ne_bytes([b0, b1, b2, b3]);
    let h2 = u32::from_ne_bytes([b4, b5, b6, b7]) & 0x7fffffff;

    // MurmurHash64B finalizer 单点见 murmur_hash_64b
    // ... truncated to 32-bit output (normally hash is equal to (uint64_t(h1) << 32) | h2, but we only really need the lower 32-bit half)
    let h2 = murmur_hash_64b(h1, h2);

    gnode!(t, lmod!(h2 as i32, sizenode!(t)) as usize)
  }
}
