use core::ffi::c_void;

use crate::{
  functions::{c_file_write, c_file_write_bytes, dump_json_head, dumpref::dumpref},
  macros::sizeudata::sizeudata,
  records::{gc_object::GCObject, udata::Udata},
};

/// # Safety
/// `f` 须为有效输出目标（对应 cpp `FILE*`，可写）；`u` 须为存活 `Udata`，读其 `memcat/len/tag`；
/// `u.metatable` 允许为 NULL，仅当非空时才 `dumpref` 递归引用该表。cpp `lgcdebug.cpp:453`。
pub(crate) unsafe fn dumpudata(f: *mut c_void, u: *mut Udata) {
  unsafe {
    let u = &*u;
    dump_json_head(f, "userdata", u.memcat, sizeudata(u.len as usize) as i32);
    c_file_write(f, format_args!(",\"tag\":{}", u.tag as i32));

    if !u.metatable.is_null() {
      c_file_write_bytes(f, b",\"metatable\":");
      let mt = u.metatable as *mut GCObject;
      dumpref(f, mt);
    }

    c_file_write_bytes(f, b"}");
  }
}
