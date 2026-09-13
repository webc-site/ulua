use core::ffi::c_void;

use crate::{
  functions::lua_d_growstack::lua_d_growstack, records::call_context_lapi::CallContext,
  type_aliases::lua_state::lua_State,
};

impl CallContext {
  pub(crate) unsafe extern "C-unwind" fn run_mut(l: *mut lua_State, ud: *mut c_void) {
    unsafe {
      let ctx = ud as *mut CallContext;
      lua_d_growstack(l, (*ctx).size);
    }
  }
}
