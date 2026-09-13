//! Node: `cxx:Function:Luau.VM:VM/src/ltable.cpp:657:lua_h_getstr`
//! Source: `VM/src/ltable.cpp` (ltable.cpp:657-669, hand-ported)

use crate::{
  macros::{
    gkey::{gkey, gval},
    gnext::gnext,
    hashstr::hashstr,
    lua_o_nilobject::luaO_nilobject,
    tsvalue::tsvalue,
    ttisstring::ttisstring,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_h_getstr(t: *mut LuaTable, key: *mut tstring) -> *const TValue {
  unsafe {
    let mut n: *mut LuaNode = hashstr!(t, key);
    loop {
      // check whether `key' is somewhere in the chain
      if ttisstring!(gkey!(n)) && tsvalue!(gkey!(n)) == key {
        return gval!(n); // that's it
      }
      if gnext!(n) == 0 {
        break;
      }
      n = n.offset(gnext!(n) as isize);
    }
    luaO_nilobject
  }
}

pub use lua_h_getstr as luaH_getstr;
