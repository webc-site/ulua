use core::ffi::c_void;

use crate::{
  macros::{gnode::gnode, lmod::lmod, sizenode::sizenode},
  records::{lua_node::LuaNode, lua_table::LuaTable},
};

/// # Safety
/// `t` 须为存活 `LuaTable` 且 node 区长度与 `lsizenode` 自洽（空表指向 dummynode 亦合法）；
/// 返回值仅在该次 rehash 前有效。`p` 只参与哈希、不被解引用。违反 `sizenode` 失真会按错误掩码取节点、越界读。
/// cpp ltable.cpp:72。
pub(crate) unsafe fn hashpointer(t: *const LuaTable, p: *const c_void) -> *mut LuaNode {
  // Discard high 32-bit portion on 64-bit platforms as it doesn't carry much entropy.
  let mut h: u32 = (p as usize) as u32;

  // MurmurHash3 32-bit finalizer
  h ^= h >> 16;
  h = h.wrapping_mul(0x85eb_ca6b_u32);
  h ^= h >> 13;
  h = h.wrapping_mul(0xc2b2_ae35_u32);
  h ^= h >> 16;

  // Safety: 契约保证 `t` 为存活 LuaTable，gnode!/sizenode! 宏读其 node 区并按 sizenode 取模索引不越界
  unsafe { gnode!(t, lmod!(h as i32, sizenode!(t)) as usize) }
}
