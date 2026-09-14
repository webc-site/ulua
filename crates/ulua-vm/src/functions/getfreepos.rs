use core::ptr::null_mut;

use crate::{
  macros::{gkey::gkey, gnode::gnode, ttisnil::ttisnil},
  records::{lua_node::LuaNode, lua_table::LuaTable},
};

pub(crate) unsafe fn getfreepos(t: *mut LuaTable) -> *mut LuaNode {
  unsafe {
    // In the C++ source, lastfree is accessed as t->lastfree.
    // In the Rust LuaTable record, lastfree is part of the union.
    // We access it through the union field.
    while (*t).union.lastfree > 0 {
      (*t).union.lastfree -= 1;

      let n = gnode!(t, (*t).union.lastfree);
      if ttisnil!(gkey!(n)) {
        return n;
      }
    }
    null_mut() // could not find a free place
  }
}
