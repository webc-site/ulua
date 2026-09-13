use core::ffi::c_void;

use crate::{
  functions::lua_s_resize::luaS_resize, records::call_context_lgc_alt_d::CallContext,
  type_aliases::lua_state::lua_State,
};

impl CallContext {
  pub(crate) unsafe extern "C-unwind" fn run(l: *mut lua_State, ud: *mut c_void) {
    unsafe {
      let ctx = ud as *mut CallContext;
      luaS_resize(l, (*ctx).newsize);
    }
  }
}
