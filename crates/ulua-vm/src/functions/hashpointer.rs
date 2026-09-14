use core::ffi::c_void;

use crate::{
  macros::{gnode::gnode, lmod::lmod, sizenode::sizenode},
  type_aliases::{lua_node::LuaNode, lua_table::LuaTable},
};

pub(crate) unsafe fn hashpointer(t: *const LuaTable, p: *const c_void) -> *mut LuaNode {
  // Discard high 32-bit portion on 64-bit platforms as it doesn't carry much entropy.
  let mut h: u32 = (p as usize) as u32;

  // MurmurHash3 32-bit finalizer
  h ^= h >> 16;
  h = h.wrapping_mul(0x85eb_ca6b_u32);
  h ^= h >> 13;
  h = h.wrapping_mul(0xc2b2_ae35_u32);
  h ^= h >> 16;

  unsafe { gnode!(t, lmod!(h as i32, sizenode!(t)) as usize) }
}
