use core::ptr::null_mut;

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_vm::{
  functions::lua_newstate::lua_newstate,
  records::{Closure::Closure, Proto::Proto, lua_state::lua_State},
};

use crate::common::{functions::alloc::alloc as luau_alloc, type_aliases::state_ref::StateRef};
pub struct FeedbackVectorFixture {
  pub bcb: BytecodeBuilder,
  pub l: StateRef,
  pub on_inline: Option<
    unsafe extern "C-unwind" fn(*mut lua_State, *mut Closure, *mut Closure, u32) -> *mut Proto,
  >,
}

impl FeedbackVectorFixture {
  pub fn new() -> Self {
    let state = unsafe { lua_newstate(Some(luau_alloc), null_mut()) };
    let l = StateRef::new(state).expect("lua_newstate failed");

    Self {
      bcb: BytecodeBuilder::new(None),
      l,
      on_inline: None,
    }
  }

  pub fn lua_state(&self) -> *mut lua_State {
    self.l.as_ptr()
  }
}

impl Default for FeedbackVectorFixture {
  fn default() -> Self {
    Self::new()
  }
}
