use crate::{
  macros::curr_func::curr_func,
  records::{lua_state::lua_State, lua_table::LuaTable},
};

pub(crate) unsafe fn getcurrenv(l: *mut lua_State) -> *mut LuaTable {
  unsafe {
    if (*l).ci == (*l).base_ci {
      (*l).gt
    } else {
      (*curr_func!(l)).env
    }
  }
}
