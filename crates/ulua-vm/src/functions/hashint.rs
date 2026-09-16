use core::ptr::copy_nonoverlapping;

use crate::{
  macros::{gnode::gnode, lmod::lmod, sizenode::sizenode},
  records::{lua_node::LuaNode, lua_table::LuaTable},
};

pub(crate) unsafe fn hashint(t: *const LuaTable, n: i64) -> *mut LuaNode {
  unsafe {
    // static_assert(sizeof(n) == sizeof(unsigned int) * 2, "expected a 8-byte integer");
    let mut i: [u32; 2] = [0; 2];
    copy_nonoverlapping(&n as *const i64 as *const u8, i.as_mut_ptr() as *mut u8, 8);

    let mut h1 = i[0];
    let mut h2 = i[1];

    // finalizer from MurmurHash64B
    const M: u32 = 0x5bd1e995;

    h1 ^= h2 >> 18;
    h1 = h1.wrapping_mul(M);
    h2 ^= h1 >> 22;
    h2 = h2.wrapping_mul(M);
    h1 ^= h2 >> 17;
    h1 = h1.wrapping_mul(M);
    h2 ^= h1 >> 19;
    h2 = h2.wrapping_mul(M);

    // ... truncated to 32-bit output (normally hash is equal to (uint64_t(h1) << 32) | h2, but we only really need the lower 32-bit half)
    // We cast h2 to i32 to satisfy the lmod! macro's expectation of signed bitwise operands in the VM's specific lmod implementation.
    gnode!(t, lmod!(h2 as i32, sizenode!(t)) as usize)
  }
}
