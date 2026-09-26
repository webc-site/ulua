use core::{ffi::c_void, ptr::eq};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{lua_h_getnum::lua_h_getnum, maybesetaboundary::maybesetaboundary},
  macros::{dummynode::dummynode, getaboundary::getaboundary},
  records::lua_table::LuaTable,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `t` 须指向存活 `LuaTable`；`boundary` 由 `getaboundary` 派生且 < `sizearray`，故对
/// `array[boundary-2 .. boundary+1]` 的读写均在数组界内（cpp ltable.cpp:updateaboundary）。
unsafe fn updateaboundary(t: *mut LuaTable, boundary: i32) -> i32 {
  unsafe {
    if boundary < (*t).sizearray && (*(*t).array.add((boundary - 1) as usize)).is_nil() {
      if boundary >= 2 && !(*(*t).array.add((boundary - 2) as usize)).is_nil() {
        maybesetaboundary(t, boundary - 1);
        return boundary - 1;
      }
    } else if boundary + 1 < (*t).sizearray
      && !(*(*t).array.add(boundary as usize)).is_nil()
      && (*(*t).array.add((boundary + 1) as usize)).is_nil()
    {
      maybesetaboundary(t, boundary + 1);
      return boundary + 1;
    }

    0
  }
}

/// # Safety
/// `t` 须指向存活 `LuaTable` 且 `array` 与 `sizearray`、hash 部分与 `sizenode` 元数据一致
/// （cpp ltable.cpp:1333）：数组部分二分回探仅在 `array[0..sizearray)` 内读，缓存 boundary
/// 的回写经 `maybesetaboundary` 维护。
pub unsafe fn lua_h_getn(t: *mut LuaTable) -> i32 {
  unsafe {
    let boundary = getaboundary(t);

    if boundary > 0 {
      if !(*(*t).array.add(((*t).sizearray - 1) as usize)).is_nil() && eq((*t).node, dummynode) {
        return (*t).sizearray;
      }

      if boundary < (*t).sizearray
        && !(*(*t).array.add((boundary - 1) as usize)).is_nil()
        && (*(*t).array.add(boundary as usize)).is_nil()
      {
        return boundary;
      }

      let foundboundary = updateaboundary(t, boundary);
      if foundboundary > 0 {
        return foundboundary;
      }
    }

    let j = (*t).sizearray;

    if j > 0 && (*(*t).array.add((j - 1) as usize)).is_nil() {
      let mut base: *mut TValue = (*t).array;
      let mut rest = j;

      while rest >> 1 != 0 {
        let half = rest >> 1;
        if !(*base.add(half as usize)).is_nil() {
          base = base.add(half as usize);
        }
        rest -= half;
      }

      let boundary = if !(*base).is_nil() { 1 } else { 0 } + base.offset_from((*t).array) as i32;
      maybesetaboundary(t, boundary);
      boundary
    } else {
      LUAU_ASSERT!(eq((*t).node, dummynode) || (*lua_h_getnum(t, j + 1)).is_nil());
      j
    }
  }
}

/// # Safety
/// C ABI 导出壳：`t` 还原为 `*mut LuaTable` 后须满足 [`lua_h_getn`] 的存活表契约。
pub unsafe extern "C-unwind" fn lua_h_getn_export(t: *mut c_void) -> i32 {
  unsafe { lua_h_getn(t as *mut LuaTable) }
}
