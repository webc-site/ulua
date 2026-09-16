use crate::type_aliases::lua_state::lua_State;
#[derive(Debug)]
#[repr(C)]
pub struct TempBuffer<T> {
  pub(crate) l: *mut lua_State,
  pub(crate) data: *mut T,
  pub(crate) count: usize,
}
