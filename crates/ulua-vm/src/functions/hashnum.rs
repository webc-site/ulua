use crate::{
  macros::{gnode::gnode, lmod::lmod, sizenode::sizenode},
  type_aliases::{lua_node::LuaNode as LuaNodeAlias, lua_table::LuaTable as LuaTableAlias},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn hashnum(t: *mut LuaTableAlias, n: f64) -> *mut LuaNodeAlias {
  unsafe {
    // static_assert(sizeof(double) == sizeof(unsigned int) * 2, "expected a 8-byte double");
    // to_bits 即语言保证的位重解释，替代 C 的 memcpy 双字别名访问（本机字节序）
    let [b0, b1, b2, b3, b4, b5, b6, b7] = n.to_bits().to_ne_bytes();
    // mask out sign bit to make sure -0 and 0 hash to the same value
    let h1 = u32::from_ne_bytes([b0, b1, b2, b3]);
    let h2 = u32::from_ne_bytes([b4, b5, b6, b7]) & 0x7fffffff;

    // finalizer from MurmurHash64B
    const M: u32 = 0x5bd1e995;

    let mut h1 = h1 ^ (h2 >> 18);
    h1 = h1.wrapping_mul(M);
    let mut h2 = h2 ^ (h1 >> 22);
    h2 = h2.wrapping_mul(M);
    let mut h1 = h1 ^ (h2 >> 17);
    h1 = h1.wrapping_mul(M);
    let h2 = h2 ^ (h1 >> 19);
    let h2 = h2.wrapping_mul(M);

    // ... truncated to 32-bit output (normally hash is equal to (uint64_t(h1) << 32) | h2, but we only really need the lower 32-bit half)
    gnode!(t, lmod!(h2 as i32, sizenode!(t)) as usize)
  }
}
