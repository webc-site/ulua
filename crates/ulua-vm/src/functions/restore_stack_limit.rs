use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_d_realloc_ci::lua_d_realloc_ci,
  macros::{extra_stack::EXTRA_STACK, luai_maxcalls::LUAI_MAXCALLS},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn restore_stack_limit(l: *mut LuaState) {
  unsafe {
    LUAU_ASSERT!(
      (*l).stack_last.offset_from((*l).stack) == ((*l).stacksize - EXTRA_STACK) as isize
    );
    if (*l).size_ci > LUAI_MAXCALLS {
      let inuse = (*l).ci.offset_from((*l).base_ci) as i32;
      if inuse + 1 < LUAI_MAXCALLS {
        lua_d_realloc_ci(l, LUAI_MAXCALLS);
      }
    }
  }
}
