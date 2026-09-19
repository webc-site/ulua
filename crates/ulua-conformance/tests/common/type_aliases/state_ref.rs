use core::ptr::NonNull;

/// A RAII wrapper around a `lua_State*` that automatically closes the state on drop.
/// Mirrors `std::unique_ptr<lua_State, void (*)(lua_State*)>` from C++.
use ulua_vm::functions::lua_close::lua_close;
use ulua_vm::records::lua_state::lua_State;
#[repr(transparent)]
pub struct StateRef(pub NonNull<lua_State>);

impl StateRef {
  /// Creates a new `StateRef` from a non-null `lua_State*` pointer.
  /// The caller is responsible for ensuring the pointer was allocated with `lua_newstate`.
  pub fn new(state: *mut lua_State) -> Option<Self> {
    NonNull::new(state).map(Self)
  }

  /// Returns the raw `lua_State*` pointer.
  pub fn as_ptr(&self) -> *mut lua_State {
    self.0.as_ptr()
  }
}

impl Drop for StateRef {
  fn drop(&mut self) {
    unsafe {
      lua_close(self.as_ptr());
    }
  }
}
