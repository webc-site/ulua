use core::ffi::{c_int, c_void};

use crate::{
  functions::{c_slice, dumpref::dumpref},
  macros::{gcvalue::gcvalue, iscollectable::iscollectable},
  type_aliases::t_value::TValue,
};

pub(crate) unsafe fn dumprefs(f: *mut c_void, data: *mut TValue, size: usize) {
  unsafe {
    let mut first = true;

    // SAFETY：data 指向调用方给定的 size 个 TValue（dump 遍历只读）。
    for val in c_slice(data, size) {
      if iscollectable!(val) {
        if !first {
          unsafe extern "C" {
            fn fputc(c: c_int, stream: *mut c_void) -> c_int;
          }
          fputc(',' as c_int, f);
        }
        first = false;

        dumpref(f, gcvalue!(val));
      }
    }
  }
}
