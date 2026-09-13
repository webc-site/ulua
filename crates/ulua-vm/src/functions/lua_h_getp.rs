use core::ffi::c_void;

use crate::{
  functions::{hashpointer::hashpointer, lua_a_toobject::luaO_nilobject},
  macros::{
    gkey::gkey, lightuserdatatag::lightuserdatatag, pvalue::pvalue,
    ttislightuserdata::ttislightuserdata,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_h_getp(t: *mut LuaTable, key: *mut c_void, tag: i32) -> *const TValue {
  unsafe {
    let mut n: *mut LuaNode = hashpointer(t as *const _, key);
    loop {
      let nk = gkey!(n);
      if ttislightuserdata!(nk) && pvalue!(nk) == key && lightuserdatatag!(nk) == tag {
        return &(*n).val;
      }
      let next_offset = (*n).key.next();
      if next_offset == 0 {
        break;
      }
      n = n.offset(next_offset as isize);
    }
    luaO_nilobject
  }
}

pub use lua_h_getp as luaH_getp;
