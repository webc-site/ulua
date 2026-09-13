use core::{ffi::c_int, ptr};

use crate::{
  functions::c_slice,
  type_aliases::{loc_var::LocVar, proto::Proto},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_f_getlocal(
  func: *const Proto,
  mut local_number: c_int,
  pc: c_int,
) -> *const LocVar {
  if func.is_null() {
    return ptr::null();
  }

  unsafe {
    for loc in c_slice((*func).locvars, (*func).sizelocvars as usize) {
      if pc >= loc.startpc && pc < loc.endpc {
        local_number -= 1;
        if local_number == 0 {
          return loc as *const LocVar;
        }
      }
    }
  }

  ptr::null()
}

pub use lua_f_getlocal as luaF_getlocal;
