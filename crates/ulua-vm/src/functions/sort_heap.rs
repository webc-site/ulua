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
  // SAFETY: 契约保证索引界内；块内经 siftheap/swap 的下标恒在 [low, u] 闭区间
  unsafe {
    LUAU_ASSERT!(low <= u);
    let count = u - low + 1;

    // 索引型 while 计数改迭代器降序窗（判定准绳「索引 for 循环改迭代器」）：
    // 访问序与原 `i = 起..=0` 逐位一致，count==1 时 `0..=-1` 空区间不迭代，
    // count>=2 时降序覆盖 count/2-1..=0，与原循环等真值（含末次后 `i-=1` 出界）。
    for i in (0..=count / 2 - 1).rev() {
      sort_siftheap(l, t, low, u, pred, i);
    }

    for i in (1..=count - 1).rev() {
      sort_swap(l, t, low, low + i);
      sort_siftheap(l, t, low, low + i - 1, pred, 0);
    }
  }
}
