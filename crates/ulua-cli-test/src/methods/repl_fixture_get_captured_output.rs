use alloc::string::String;
use core::{
  ffi::{CStr, c_char},
  ptr::null_mut,
};

use ulua_vm::{
  functions::lua_tolstring::lua_tolstring,
  macros::{lua_getglobal::lua_getglobal, lua_pop::lua_pop},
};

use crate::records::repl_fixture::ReplFixture;

impl ReplFixture {
  pub fn get_captured_output(&mut self) -> String {
    unsafe {
      lua_getglobal(
        self.l as *mut _,
        c"capturedoutput".as_ptr() as *const c_char,
      );
      let str_ptr = lua_tolstring(self.l as *mut _, -1, null_mut());
      let result = if str_ptr.is_null() {
        String::new()
      } else {
        let cs = CStr::from_ptr(str_ptr);
        cs.to_string_lossy().into_owned()
      };
      lua_pop(self.l as *mut _, 1);
      result
    }
  }
}
