use core::ffi::c_int;

use crate::{
  functions::{
    lua_h_getnum::lua_h_getnum, lua_h_getstr::lua_h_getstr, lua_o_rawequal_key::luaO_rawequalKey,
    mainposition::mainposition,
  },
  macros::{
    cast_num::cast_num,
    gkey::{gkey, gval},
    lua_o_nilobject::luaO_nilobject,
    luai_numeq::luai_numeq,
    nvalue::nvalue,
    tsvalue::tsvalue,
    ttype::ttype,
  },
  records::lua_table::LuaTable,
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_h_get(t: *mut LuaTable, key: *const TValue) -> *const TValue {
  unsafe {
    let tt = ttype!(key);
    match tt {
      0 => luaO_nilobject,
      6 => lua_h_getstr(t, tsvalue!(key) as *mut _),
      3 => {
        let n = nvalue!(key);
        let k = n as c_int;
        if luai_numeq(cast_num!(k), nvalue!(key)) {
          return lua_h_getnum(t, k);
        }
        let mut n = mainposition(t, key);
        loop {
          if luaO_rawequalKey(gkey!(n), key) != 0 {
            return gval!(n);
          }
          let next = (*n).key.next();
          if next == 0 {
            break;
          }
          n = n.offset(next as isize);
        }
        luaO_nilobject
      }
      _ => {
        let mut n = mainposition(t, key);
        loop {
          if luaO_rawequalKey(gkey!(n), key) != 0 {
            return gval!(n);
          }
          let next = (*n).key.next();
          if next == 0 {
            break;
          }
          n = n.offset(next as isize);
        }
        luaO_nilobject
      }
    }
  }
}

pub use lua_h_get as luaH_get;
