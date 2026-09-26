use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{sort_siftheap::sort_siftheap, sort_swap::sort_swap},
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::sort_predicate::SortPredicate,
};

/// # Safety
///
/// `l`/`t`/`pred` 须满足 `sort_siftheap`/`sort_swap` 的入约（存活调用帧与表、
/// `low <= u < sizearray`、谓词不改表）；本函数在 `[low, u]` 闭区间内自建堆再逐次
/// 归顶，越出该契约时堆化下标会落到数组段之外。
/// cpp ltablib.cpp:465 `sort_heap`。
///
/// [`sort_siftheap`]: ../sort_siftheap/fn.sort_siftheap.html
/// [`sort_swap`]: ../sort_swap/fn.sort_swap.html
pub(crate) unsafe fn sort_heap(
  l: *mut LuaState,
  t: *mut LuaTable,
  low: i32,
  u: i32,
  pred: SortPredicate,
) {
  // Safety: 契约保证索引界内；块内经 siftheap/swap 的下标恒在 [low, u] 闭区间
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
