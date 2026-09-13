use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_d_realloc_ci::lua_d_realloc_ci,
  macros::{cast_int::cast_int, extra_stack::EXTRA_STACK, luai_maxcalls::LUAI_MAXCALLS},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn restore_stack_limit(l: *mut lua_State) {
  unsafe {
    LUAU_ASSERT!(
      (*l).stack_last.offset_from((*l).stack) == ((*l).stacksize - EXTRA_STACK) as isize
    );
    if (*l).size_ci > LUAI_MAXCALLS {
      let inuse = cast_int!((*l).ci.offset_from((*l).base_ci));
      if inuse + 1 < LUAI_MAXCALLS {
        lua_d_realloc_ci(l, LUAI_MAXCALLS);
      }
    }
  }
}
