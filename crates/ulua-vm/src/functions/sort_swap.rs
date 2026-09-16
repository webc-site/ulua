use core::mem::zeroed;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::{setobj_2_s::setobj_2_s, setobj_2_t::setobj2t},
  records::lua_table::LuaTable,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

#[inline]
pub(crate) unsafe fn sort_swap(l: *mut lua_State, t: *mut LuaTable, i: i32, j: i32) {
  unsafe {
    let arr = (*t).array;
    let n = (*t).sizearray;

    LUAU_ASSERT!((i as u32) < (n as u32) && (j as u32) < (n as u32));

    let mut temp: TValue = zeroed();
    setobj_2_s!(l, &mut temp as *mut TValue, arr.add(i as usize));
    setobj2t!(l, arr.add(i as usize), arr.add(j as usize));
    setobj2t!(l, arr.add(j as usize), &temp as *const TValue);
  }
}
