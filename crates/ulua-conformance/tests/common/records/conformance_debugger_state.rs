use core::{
  ptr::null_mut,
  sync::atomic::{AtomicBool, AtomicI32, AtomicPtr, Ordering},
};

use ulua_vm::records::lua_state::lua_State;
#[derive(Debug)]
pub struct ConformanceDebuggerState {
  pub breakhits: AtomicI32,
  pub interruptedthread: AtomicPtr<lua_State>,
  pub singlestep: AtomicBool,
  pub stephits: AtomicI32,
}

impl ConformanceDebuggerState {
  pub const fn new() -> Self {
    Self {
      breakhits: AtomicI32::new(0),
      interruptedthread: AtomicPtr::new(null_mut()),
      singlestep: AtomicBool::new(false),
      stephits: AtomicI32::new(0),
    }
  }

  pub fn reset(&self, singlestep: bool) {
    self.breakhits.store(0, Ordering::SeqCst);
    self.interruptedthread.store(null_mut(), Ordering::SeqCst);
    self.singlestep.store(singlestep, Ordering::SeqCst);
    self.stephits.store(0, Ordering::SeqCst);
  }
}

impl Default for ConformanceDebuggerState {
  fn default() -> Self {
    Self::new()
  }
}

pub static CONFORMANCE_DEBUGGER_STATE: ConformanceDebuggerState = ConformanceDebuggerState::new();
