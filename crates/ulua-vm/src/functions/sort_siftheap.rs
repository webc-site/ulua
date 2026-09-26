use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{sort_less::sort_less, sort_swap::sort_swap},
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::sort_predicate::SortPredicate,
};

/// # Safety
///
/// `l`/`t`/`pred` 须满足 `sort_less`/`sort_swap` 的入约（存活调用帧与表、谓词不改表），
/// 且 `low <= u < sizearray`、`0 <= root`；下沉路径上的下标由 heap 不变式钳在
/// `[low, u]` 闭区间内，失配时会读越数组段。cpp ltablib.cpp:439 `sort_siftheap`。
pub(crate) unsafe fn sort_siftheap(
  l: *mut LuaState,
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

    // Safety: 契约+heap 不变式保证 low+next/left/right 均在 [low, u] 界内
    next = if unsafe { sort_less(l, t, low + next, low + left, pred) } != 0 {
      left
    } else {
      next
    };
    // Safety: next/right 由 [low, u] 区间钳制，`sort_less` 的入约（存活帧与表）与上方同一契约已保证
    next = if unsafe { sort_less(l, t, low + next, low + right, pred) } != 0 {
      right
    } else {
      next
    };

    if next == root {
      break;
    }

    // Safety: 两下标同落在 [low, u] 界内（上方 sort_less 已访过同对）
    unsafe {
      sort_swap(l, t, low + root, low + next);
    }
    root = next;
  }

  // process last element if it has just one child
  let lastleft = root * 2 + 1;
  // Safety: 单孩子收尾分支的 low+root/low+lastleft 仍在 [low, u] 界内
  if lastleft == count - 1 && unsafe { sort_less(l, t, low + root, low + lastleft, pred) } != 0 {
    unsafe {
      sort_swap(l, t, low + root, low + lastleft);
    }
  }
}
