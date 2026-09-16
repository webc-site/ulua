use core::slice::from_raw_parts_mut;

use crate::{
  functions::countint::countint,
  macros::{
    gkey::{gkey, gval},
    maxbits::MAXBITS,
    nvalue::nvalue,
    sizenode::sizenode,
    ttisnil::ttisnil,
    ttisnumber::ttisnumber,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn numusehash(t: *const LuaTable, nums: *mut i32, pnasize: *mut i32) -> i32 {
  let mut totaluse: i32 = 0; // total number of elements
  let mut ause: i32 = 0; // summation of `nums'
  let mut i: i32 = unsafe { sizenode!(t) };

  while i != 0 {
    i -= 1;

    let n: *mut LuaNode = unsafe { (*t).node.add(i as usize) };
    unsafe {
      if !ttisnil!(gval!(n)) {
        if ttisnumber!(gkey!(n)) {
          let key = nvalue!(gkey!(n));
          ause += countint(key, from_raw_parts_mut(nums, (MAXBITS + 1) as usize));
        }
        totaluse += 1;
      }
    }
  }

  unsafe {
    *pnasize += ause;
  }
  totaluse
}
