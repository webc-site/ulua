use core::ffi::c_int;

use crate::{
  functions::{c_slice, lua_g_getline::luaG_getline},
  records::proto::Proto,
};

pub(crate) unsafe fn getmaxline(p: *mut Proto) -> c_int {
  unsafe {
    let mut result: i32 = -1;

    for i in 0..(*p).sizecode {
      result = result.max(luaG_getline(p, i));
    }

    for &sub in c_slice((*p).p, (*p).sizep as usize) {
      result = result.max(getmaxline(sub));
    }

    result
  }
}
