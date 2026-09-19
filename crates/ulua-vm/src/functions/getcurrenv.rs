use crate::{
  macros::curr_func::curr_func,
  records::{lua_state::lua_State, lua_table::LuaTable},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn getcurrenv(l: *mut lua_State) -> *mut LuaTable {
  unsafe {
    if (*l).ci == (*l).base_ci {
      (*l).gt
    } else {
      (*curr_func!(l)).env
    }
  }
}
