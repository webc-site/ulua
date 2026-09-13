use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_l_error_l::lua_l_error_l,
  type_aliases::{lua_state::lua_State, lua_table::LuaTable, sort_predicate::SortPredicate},
};

#[inline]
pub(crate) unsafe fn sort_less(
  l: *mut lua_State,
  t: *mut LuaTable,
  i: i32,
  j: i32,
  pred: SortPredicate,
) -> i32 {
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
      lua_l_error_l(
        l,
        c"table modified during sorting".as_ptr(),
        core::format_args!("table modified during sorting"),
      );
    }

    res
  }
}
