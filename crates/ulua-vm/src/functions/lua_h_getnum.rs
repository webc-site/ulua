use core::ffi::{c_int, c_uint};

use crate::{
  functions::{hashnum::hashnum, lua_a_toobject::luaO_nilobject},
  macros::{
    cast_num::cast_num,
    dummynode::luaH_dummynode,
    gkey::{gkey, gval},
    luai_numeq::luai_numeq,
    nvalue::nvalue,
    ttisnumber::ttisnumber,
  },
  records::lua_table::LuaTable,
  type_aliases::t_value::TValue,
};

pub(crate) unsafe fn lua_h_getnum(t: *mut LuaTable, key: c_int) -> *const TValue {
  unsafe {
    // (1 <= key && key <= t->sizearray)
    if (key as c_uint).wrapping_sub(1) < (*t).sizearray as c_uint {
      (*t).array.add((key - 1) as usize)
    } else if (*t).node != &luaH_dummynode as *const _ as *mut _ {
      let nk = cast_num!(key);
      let mut n = hashnum(t, nk);

      loop {
        // check whether `key' is somewhere in the chain
        if ttisnumber!(gkey!(n)) && luai_numeq(nvalue!(gkey!(n)), nk) {
          return gval!(n); // that's it
        }

        // gnext(n) is defined as ((n)->key.next) in ltable.h
        let next_offset = (*n).key.next();

        if next_offset == 0 {
          break;
        }
        n = n.offset(next_offset as isize);
      }
      luaO_nilobject
    } else {
      luaO_nilobject
    }
  }
}
