use ulua_vm::{macros::lua_pop::lua_pop, type_aliases::lua_state::lua_State};

#[derive(Debug)]
pub(crate) struct ThreadPopper {
  pub(crate) l: *mut lua_State,
}

impl Drop for ThreadPopper {
  fn drop(&mut self) {
    unsafe {
      lua_pop(self.l, 1);
    }
  }
}
