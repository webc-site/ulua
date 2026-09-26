use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::lua_l_error::luaL_error,
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::sort_predicate::SortPredicate,
};

/// # Safety
///
/// `t` 必须指向存活 `LuaTable` 且 `0 <= i, j < sizearray`（LUAU_ASSERT 兜底，失配即把越界
/// 槽位交给谓词解引用）；`l` 必须指向存活 `LuaState`——`pred` 是可回跑的 Lua 比较函数，
/// 报错经它抛出；`pred` 为 None 或遵循 C 约定；谓词若改动 t 使 `sizearray` 变化，本函数
/// 检测后报 "table modified during sorting"，但旧下标自谓词返回起已不可信。
/// cpp ltablib.cpp:424 `sort_less`。
#[inline]
pub(crate) unsafe fn sort_less(
  l: *mut LuaState,
  t: *mut LuaTable,
  i: i32,
  j: i32,
  pred: SortPredicate,
) -> i32 {
  // Safety: 契约保证 t 存活；arr.add(i/j) 在 sizearray 界内（下方 LUAU_ASSERT 兜底后才调用）
  unsafe {
    let arr = (*t).array;
    let n = (*t).sizearray;

    LUAU_ASSERT!((i as u32) < (n as u32) && (j as u32) < (n as u32));

    let res = match pred {
      Some(f) => f(l, arr.add(i as usize), arr.add(j as usize)),
      None => 0,
    };

    // predicate call may resize the table, which is invalid
    if (*t).sizearray != n {
      luaL_error!(l, "table modified during sorting");
    }

    res
  }
}
