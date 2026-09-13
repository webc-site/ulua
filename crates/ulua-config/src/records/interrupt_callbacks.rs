use alloc::rc::Rc;
use core::{
  ffi::{c_int, c_void},
  fmt::{Debug, Formatter, Result},
};

use ulua_vm::type_aliases::lua_state::lua_State;

#[derive(Clone, Default)]
pub struct InterruptCallbacks {
  pub init_callback: Option<Rc<dyn Fn(*mut lua_State)>>,
  pub interrupt_callback: Option<unsafe extern "C-unwind" fn(l: *mut lua_State, gc: c_int)>,
}

impl Debug for InterruptCallbacks {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("InterruptCallbacks")
      .field("init_callback", &self.init_callback.as_ref().map(|_| "..."))
      .field(
        "interrupt_callback",
        &self.interrupt_callback.map(|f| f as *const c_void),
      )
      .finish()
  }
}
