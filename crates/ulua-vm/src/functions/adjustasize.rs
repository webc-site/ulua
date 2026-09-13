use core::ffi::c_int;

use crate::{
  functions::{arrayindex::arrayindex, lua_h_getnum::lua_h_getnum},
  macros::{dummynode::luaH_dummynode, nvalue::nvalue, ttisnil::ttisnil, ttisnumber::ttisnumber},
  type_aliases::{lua_table::LuaTable, t_value::TValue},
};

pub(crate) unsafe fn adjustasize(t: *mut LuaTable, mut size: c_int, ek: *const TValue) -> c_int {
  unsafe {
    let tbound = (*t).node != &luaH_dummynode as *const _ as *mut _ || size < (*t).sizearray;
    let ekindex = if !ek.is_null() && ttisnumber!(ek) {
      arrayindex(nvalue!(ek))
    } else {
      -1
    };

    // move the array size up until the boundary is guaranteed to be inside the array part.
    // Stop at INT_MAX: the array can't be larger, and `size + 1` there overflows `int`
    // (UB in C++ Luau / a panic with overflow-checks on — found by the run fuzz target).
    while size != c_int::MAX
      && (size + 1 == ekindex || (tbound && !ttisnil!(lua_h_getnum(t, size + 1))))
    {
      size += 1;
    }

    size
  }
}
