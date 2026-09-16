use core::ffi::c_void;

use crate::{
  functions::{c_file_write_bytes, c_slice, dumpref::dumpref},
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
          c_file_write_bytes(f, b",");
        }
        first = false;

        dumpref(f, gcvalue!(val));
      }
    }
  }
}
