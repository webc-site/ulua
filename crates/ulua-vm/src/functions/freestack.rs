use crate::{
  macros::lua_m_freearray::luaM_freearray,
  type_aliases::{call_info::CallInfo, lua_state::lua_State, t_value::TValue},
};

pub(crate) unsafe fn freestack(l: *mut lua_State, l1: *mut lua_State) {
  unsafe {
    luaM_freearray!(
      l,
      (*l1).base_ci,
      (*l1).size_ci as usize,
      CallInfo,
      (*l1).activememcat
    );
    luaM_freearray!(
      l,
      (*l1).stack,
      (*l1).stacksize as usize,
      TValue,
      (*l1).activememcat
    );
  }
}
