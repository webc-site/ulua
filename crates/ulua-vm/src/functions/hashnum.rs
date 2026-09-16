use crate::{
  macros::{gnode::gnode, lmod::lmod, sizenode::sizenode},
  type_aliases::{lua_node::LuaNode as LuaNodeAlias, lua_table::LuaTable as LuaTableAlias},
};

pub(crate) unsafe fn hashnum(t: *mut LuaTableAlias, n: f64) -> *mut LuaNodeAlias {
  unsafe {
    // static_assert(sizeof(double) == sizeof(unsigned int) * 2, "expected a 8-byte double");
    // to_bits 即语言保证的位重解释，替代 C 的 memcpy 双字别名访问（本机字节序）
    let bits = n.to_bits().to_ne_bytes();
    // mask out sign bit to make sure -0 and 0 hash to the same value
    let h1 = u32::from_ne_bytes(bits[0..4].try_into().unwrap());
    let h2 = u32::from_ne_bytes(bits[4..8].try_into().unwrap()) & 0x7fffffff;

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
