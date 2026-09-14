use core::mem::zeroed;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{c_slice, validateobjref::validateobjref, validateref::validateref},
  macros::{
    gkey::{gkey, gval},
    ttisnil::ttisnil,
  },
  records::{
    gc_object::GCObject, global_state::global_State, lua_node::LuaNode, lua_table::LuaTable,
  },
  type_aliases::t_value::TValue,
};

pub(crate) unsafe fn validatetable(g: *mut global_State, h: *mut LuaTable) {
  unsafe {
    let sizenode = 1 << (*h).lsizenode;

    LUAU_ASSERT!((*h).union.lastfree <= sizenode);

    let h_gco = h as *mut GCObject;

    if !(*h).metatable.is_null() {
      validateobjref(g, h_gco, (*h).metatable as *mut GCObject);
    }

    // SAFETY：array 为 C 指针 + sizearray 计数，与表分配一致。
    for val in c_slice((*h).array, (*h).sizearray as usize) {
      validateref(g, h_gco, val);
    }

    for i in 0..sizenode {
      let n: *mut LuaNode = (*h).node.add(i as usize);

      // ttype(gkey(n)) -> (*gkey!(n)).tt()
      // ttisnil(gval(n)) -> ttisnil!(gval!(n))
      LUAU_ASSERT!((*gkey!(n)).tt() != LuaType::DeadKey as i32 || ttisnil!(gval!(n)));

      // gnext(n) -> (*n).key.next()
      let next_val = (*n).key.next();
      LUAU_ASSERT!(i + next_val >= 0 && i + next_val < sizenode);

      if !ttisnil!(gval!(n)) {
        let mut k: TValue = zeroed();
        k.tt = (*gkey!(n)).tt();
        k.value = (*gkey!(n)).value;

        validateref(g, h_gco, &k);
        validateref(g, h_gco, &*gval!(n));
      }
    }
  }
}
