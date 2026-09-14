use core::{
  ffi::{c_int, c_uint, c_void},
  mem::zeroed,
};

use crate::{
  functions::{lua_h_getnum::lua_h_getnum, newkey::newkey},
  macros::{
    cast_num::cast_num, cast_to::cast_to, lua_o_nilobject::luaO_nilobject, setnvalue::setnvalue,
  },
  records::lua_table::LuaTable,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_h_setnum(l: *mut lua_State, t: *mut LuaTable, key: c_int) -> *mut TValue {
  unsafe {
    // (1 <= key && key <= t->sizearray)
    if (key as c_uint).wrapping_sub(1) < (*t).sizearray as c_uint {
      return (*t).array.add((key - 1) as usize);
    }

    // hash fallback
    let p = lua_h_getnum(t, key);
    if p != luaO_nilobject {
      cast_to!(*mut TValue, p)
    } else {
      let mut k: TValue = zeroed();
      setnvalue!(&mut k, cast_num!(key));

      newkey(l, t, &k)
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaH_setnum")]
pub unsafe extern "C-unwind" fn lua_h_setnum_export(
  l: *mut lua_State,
  t: *mut c_void,
  key: c_int,
) -> *mut TValue {
  unsafe { lua_h_setnum(l, t as *mut LuaTable, key) }
}

pub use lua_h_setnum as luaH_setnum;
