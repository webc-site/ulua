use core::ffi::c_char;

use ulua_vm::{
  functions::{lua_checkstack::lua_checkstack, lua_pushlstring::lua_pushlstring},
  records::lua_state::lua_State,
};

pub(crate) unsafe fn setup_arguments(l: *mut lua_State, args: &[impl AsRef<str>]) {
  unsafe {
    lua_checkstack(l, args.len() as i32);
    for arg in args {
      let s = arg.as_ref();
      lua_pushlstring(l, s.as_ptr() as *const c_char, s.len());
    }
  }
}
