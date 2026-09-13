use core::{
  ffi::{c_int, c_void},
  ptr::eq,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_h_getnum::lua_h_getnum,
  macros::{dummynode::dummynode, getaboundary::getaboundary, ttisnil::ttisnil},
  records::lua_table::LuaTable,
  type_aliases::t_value::TValue,
};

#[inline]
unsafe fn maybesetaboundary(t: *mut LuaTable, boundary: c_int) {
  unsafe {
    if (*t).union.aboundary <= 0 {
      (*t).union.aboundary = -boundary;
    }
  }
}

unsafe fn updateaboundary(t: *mut LuaTable, boundary: c_int) -> c_int {
  unsafe {
    if boundary < (*t).sizearray && ttisnil!((*t).array.add((boundary - 1) as usize)) {
      if boundary >= 2 && !ttisnil!((*t).array.add((boundary - 2) as usize)) {
        maybesetaboundary(t, boundary - 1);
        return boundary - 1;
      }
    } else if boundary + 1 < (*t).sizearray
      && !ttisnil!((*t).array.add(boundary as usize))
      && ttisnil!((*t).array.add((boundary + 1) as usize))
    {
      maybesetaboundary(t, boundary + 1);
      return boundary + 1;
    }

    0
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_h_getn(t: *mut LuaTable) -> c_int {
  unsafe {
    let boundary = getaboundary(t);

    if boundary > 0 {
      if !ttisnil!((*t).array.add(((*t).sizearray - 1) as usize)) && eq((*t).node, dummynode) {
        return (*t).sizearray;
      }

      if boundary < (*t).sizearray
        && !ttisnil!((*t).array.add((boundary - 1) as usize))
        && ttisnil!((*t).array.add(boundary as usize))
      {
        return boundary;
      }

      let foundboundary = updateaboundary(t, boundary);
      if foundboundary > 0 {
        return foundboundary;
      }
    }

    let j = (*t).sizearray;

    if j > 0 && ttisnil!((*t).array.add((j - 1) as usize)) {
      let mut base: *mut TValue = (*t).array;
      let mut rest = j;

      while rest >> 1 != 0 {
        let half = rest >> 1;
        if !ttisnil!(base.add(half as usize)) {
          base = base.add(half as usize);
        }
        rest -= half;
      }

      let boundary = if !ttisnil!(base) { 1 } else { 0 } + base.offset_from((*t).array) as c_int;
      maybesetaboundary(t, boundary);
      boundary
    } else {
      LUAU_ASSERT!(eq((*t).node, dummynode) || ttisnil!(lua_h_getnum(t, j + 1)));
      j
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaH_getn")]
pub unsafe extern "C-unwind" fn lua_h_getn_export(t: *mut c_void) -> c_int {
  unsafe { lua_h_getn(t as *mut LuaTable) }
}

pub use lua_h_getn as luaH_getn;
