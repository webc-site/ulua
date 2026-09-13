use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{sort_siftheap::sort_siftheap, sort_swap::sort_swap},
  records::lua_table::LuaTable,
  type_aliases::{lua_state::lua_State, sort_predicate::SortPredicate},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn sort_heap(
  l: *mut lua_State,
  t: *mut LuaTable,
  low: i32,
  u: i32,
  pred: SortPredicate,
) {
  unsafe {
    LUAU_ASSERT!(low <= u);
    let count = u - low + 1;

    let mut i = count / 2 - 1;
    while i >= 0 {
      sort_siftheap(l, t, low, u, pred, i);
      i -= 1;
    }

    let mut i = count - 1;
    while i > 0 {
      sort_swap(l, t, low, low + i);
      sort_siftheap(l, t, low, low + i - 1, pred, 0);
      i -= 1;
    }
  }
}
