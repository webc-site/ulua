use core::{ffi::c_char, mem::size_of};

use crate::{
  functions::{
    gettablemode::gettablemode, removeentry::removeentry,
    tableresizeprotected::tableresizeprotected,
  },
  macros::{
    gkey::{gkey, gval},
    gnode::gnode,
    iscleared::iscleared,
    setnilvalue::setnilvalue,
    sizenode::sizenode,
    ttisnil::ttisnil,
  },
  records::{gc_object::GCObject, lua_node::LuaNode, lua_table::LuaTable},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

#[inline]
unsafe fn contains_s(mut mode: *const c_char) -> bool {
  unsafe {
    while !mode.is_null() && *mode != 0 {
      if *mode == b's' as c_char {
        return true;
      }
      mode = mode.add(1);
    }
    false
  }
}

pub(crate) unsafe fn cleartable(l: *mut lua_State, mut list: *mut GCObject) -> usize {
  unsafe {
    let mut work = 0usize;

    while !list.is_null() {
      let h = list as *mut LuaTable;
      let hsize = sizenode!(h);
      work += size_of::<LuaTable>()
        + size_of::<TValue>() * (*h).sizearray as usize
        + size_of::<LuaNode>() * hsize as usize;

      let mut i = (*h).sizearray;
      while i > 0 {
        i -= 1;
        let o = (*h).array.add(i as usize);
        if iscleared!(o) {
          setnilvalue!(o);
        }
      }

      i = hsize;
      let mut activevalues = 0;
      while i > 0 {
        i -= 1;
        let n = gnode!(h, i);
        if !ttisnil!(gval!(n)) {
          if iscleared!(gkey!(n)) || iscleared!(gval!(n)) {
            setnilvalue!(gval!(n));
            removeentry(n);
          } else {
            activevalues += 1;
          }
        }
      }

      let modev = gettablemode((*l).global, h);
      if !modev.is_null() && contains_s(modev) && activevalues < hsize * 3 / 8 {
        tableresizeprotected(l, h, activevalues);
      }

      list = (*h).gclist;
    }

    work
  }
}
