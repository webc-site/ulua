use core::ffi::c_void;

use crate::{
  functions::{c_file_write_bytes, c_slice, dumpref::dumpref},
  macros::{gcvalue::gcvalue, iscollectable::iscollectable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `f` 须为有效可写 FILE；`data` 须指向 `size` 个连续存活的 TValue（区间界由调用方传入的元素计数给定），
/// 遍历对可回收项解引用 `gcvalue` 并交 `dumpref`。只读，不改动 GC 状态。cpp/VM/src/lgcdebug.cpp:330 dumprefs。
pub(crate) unsafe fn dumprefs(f: *mut c_void, data: *mut TValue, size: usize) {
  unsafe {
    let mut first = true;

    // Safety:data 指向调用方给定的 size 个 TValue（dump 遍历只读）。
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
