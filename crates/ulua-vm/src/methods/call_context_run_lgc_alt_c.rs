use core::ffi::c_void;

use crate::{
  functions::lua_h_resizehash::lua_h_resizehash, records::call_context_lgc_alt_c::CallContext,
  type_aliases::lua_state::lua_State,
};

impl CallContext {
  pub(crate) unsafe extern "C-unwind" fn run(l: *mut lua_State, ud: *mut c_void) {
    unsafe {
      let ctx = ud as *mut CallContext;
      lua_h_resizehash(l, (*ctx).t, (*ctx).nhsize);
    }
  }
}
