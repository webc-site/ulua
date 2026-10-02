use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{lua_h_getnum::lua_h_getnum, maybesetaboundary::maybesetaboundary},
  macros::getaboundary::getaboundary,
  records::lua_table::LuaTable,
};

/// # Safety
/// `t` 须指向存活 `LuaTable`；`boundary` 由 `getaboundary` 派生且 < `sizearray`，故对
/// 数组窗 `array_window()[boundary-2 .. boundary+1]` 的读均在数组段界内
/// （cpp ltable.cpp:updateaboundary；损坏态越界由 cpp UB 降级为切片 panic）。
unsafe fn updateaboundary(t: *mut LuaTable, boundary: i32) -> i32 {
  unsafe {
    // 数组段一律走 array_window 共享窗：窗[i] ⇔ cpp `array.add(i)`（E1 契约逐位一致）；
    // maybesetaboundary 仅写 LuaTable 头部 union 缓存，与 array 段分配不相交，窗借用不违别名。
    let arr = (*t).array_window();
    if boundary < (*t).sizearray && arr[(boundary - 1) as usize].is_nil() {
      if boundary >= 2 && !arr[(boundary - 2) as usize].is_nil() {
        maybesetaboundary(t, boundary - 1);
        return boundary - 1;
      }
    } else if boundary + 1 < (*t).sizearray
      && !arr[boundary as usize].is_nil()
      && arr[(boundary + 1) as usize].is_nil()
    {
      maybesetaboundary(t, boundary + 1);
      return boundary + 1;
    }

    0
  }
}

/// # Safety
/// `t` 须指向存活 `LuaTable` 且 `array` 与 `sizearray`、hash 部分与 `sizenode` 元数据一致
/// （cpp ltable.cpp:1333）：数组部分二分回探仅在 `array_window()` 窗内读，缓存 boundary
/// 的回写经 `maybesetaboundary` 维护。
pub unsafe fn lua_h_getn(t: *mut LuaTable) -> i32 {
  unsafe {
    let boundary = getaboundary(t);

    if boundary > 0 {
      let arr = (*t).array_window();
      if !arr[((*t).sizearray - 1) as usize].is_nil() && (*t).is_hash_dummy() {
        return (*t).sizearray;
      }

      if boundary < (*t).sizearray
        && !arr[(boundary - 1) as usize].is_nil()
        && arr[boundary as usize].is_nil()
      {
        return boundary;
      }

      let foundboundary = updateaboundary(t, boundary);
      if foundboundary > 0 {
        return foundboundary;
      }
    }

    let j = (*t).sizearray;
    let arr = (*t).array_window();

    if j > 0 && arr[(j - 1) as usize].is_nil() {
      // cpp 二分回探：base 裸指针走查收编为窗内下标（base.add(half) ⇔ base+half、
      // base.offset_from(array) ⇔ base 下标），迭代序与短路判据逐位不变。
      let mut base: usize = 0;
      let mut rest = j;

      while rest >> 1 != 0 {
        let half = rest >> 1;
        if !arr[base + half as usize].is_nil() {
          base += half as usize;
        }
        rest -= half;
      }

      let boundary = (if !arr[base].is_nil() { 1 } else { 0 }) + base as i32;
      maybesetaboundary(t, boundary);
      boundary
    } else {
      LUAU_ASSERT!((*t).is_hash_dummy() || (*lua_h_getnum(&*t, j + 1)).is_nil());
      j
    }
  }
}

/// # Safety
/// C ABI 导出壳：`t` 还原为 `*mut LuaTable` 后须满足 [`lua_h_getn`] 的存活表契约。
pub unsafe extern "C-unwind" fn lua_h_getn_export(t: *mut c_void) -> i32 {
  unsafe { lua_h_getn(t as *mut LuaTable) }
}
