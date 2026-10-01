use crate::records::lua_state::LuaState;
#[derive(Debug)]
#[repr(C)]
pub struct TempBuffer<T> {
  pub l: *mut LuaState,
  pub data: *mut T,
  pub count: usize,
}
