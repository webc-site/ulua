use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{setobj_2_s::setobj_2_s, setobj_2_t::setobj2t},
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
///
/// `t` 必须指向存活 `LuaTable` 且 `0 <= i, j < sizearray`（LUAU_ASSERT 兜底，失配即越出
/// array 段写坏堆内存）；`l` 必须指向本次排序调用的存活 `LuaState`——`setobj2t` 经它做
/// GC 写屏障记账，脱离调用帧使用会记错对象。谓词改动表导致 `array` 重分配后不得再持旧
/// 下标调本函数。cpp ltablib.cpp:411 `sort_swap`。
#[inline]
pub(crate) unsafe fn sort_swap(l: *mut LuaState, t: *mut LuaTable, i: i32, j: i32) {
  // Safety: 契约保证 i/j 在数组界内，arr.add 与 setobj 搬运均落在 array[0..sizearray)
  unsafe {
    let arr = (*t).array;
    let n = (*t).sizearray;

    LUAU_ASSERT!((i as u32) < (n as u32) && (j as u32) < (n as u32));

    let mut temp = TValue::default();
    setobj_2_s!(l, &mut temp as *mut TValue, arr.add(i as usize));
    setobj2t!(l, arr.add(i as usize), arr.add(j as usize));
    setobj2t!(l, arr.add(j as usize), &temp as *const TValue);
  }
}
