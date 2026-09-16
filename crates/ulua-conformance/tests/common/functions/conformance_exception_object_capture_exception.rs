use alloc::string::String;
use core::ffi::{CStr, c_char};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_vm::{
  functions::{
    lua_call::lua_call, lua_getfield::lua_getfield, lua_is_lfunction::lua_is_lfunction,
    lua_newthread::lua_newthread,
  },
  macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::{lua_exception::lua_exception, lua_state::lua_State},
};

use crate::common::records::exception_result::ExceptionResult;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn conformance_exception_object_capture_exception(
  l: *mut lua_State,
  function_to_run: *const c_char,
) -> ExceptionResult {
  unsafe {
    let result = catch_unwind(AssertUnwindSafe(|| {
      let thread_state = lua_newthread(l);
      lua_getfield(thread_state, LUA_GLOBALSINDEX, function_to_run);
      assert_ne!(lua_is_lfunction(thread_state, -1), 0);
      lua_call(thread_state, 0, 0);
    }));

    match result {
      Ok(()) => ExceptionResult {
        exception_generated: false,
        description: String::new(),
      },
      Err(payload) => {
        if let Some(e) = payload.downcast_ref::<lua_exception>() {
          let what = e.what();
          assert!(!what.is_null());
          ExceptionResult {
            exception_generated: true,
            description: CStr::from_ptr(what).to_string_lossy().into_owned(),
          }
        } else {
          resume_unwind(payload);
        }
      }
    }
  }
}
