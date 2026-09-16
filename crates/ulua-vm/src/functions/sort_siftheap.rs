use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{sort_less::sort_less, sort_swap::sort_swap},
  records::lua_table::LuaTable,
  type_aliases::{lua_state::lua_State, sort_predicate::SortPredicate},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn sort_siftheap(
  l: *mut lua_State,
  t: *mut LuaTable,
  low: i32,
  u: i32,
  pred: SortPredicate,
  root: i32,
) {
  LUAU_ASSERT!(low <= u);
  let count = u - low + 1;

  // process all elements with two children
  let mut root = root;
  while root * 2 + 2 < count {
    let left = root * 2 + 1;
    let right = root * 2 + 2;
    let mut next = root;

    next = if unsafe { sort_less(l, t, low + next, low + left, pred) } != 0 {
      left
    } else {
      next
    };
    next = if unsafe { sort_less(l, t, low + next, low + right, pred) } != 0 {
      right
    } else {
      next
    };

    if next == root {
      break;
    }

    unsafe {
      sort_swap(l, t, low + root, low + next);
    }
    root = next;
  }

  // process last element if it has just one child
  let lastleft = root * 2 + 1;
  if lastleft == count - 1 && unsafe { sort_less(l, t, low + root, low + lastleft, pred) } != 0 {
    unsafe {
      sort_swap(l, t, low + root, low + lastleft);
    }
  }
}
