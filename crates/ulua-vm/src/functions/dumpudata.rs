use core::ffi::{c_int, c_void};

use crate::{
  functions::{c_file_write, c_file_write_bytes, dumpref::dumpref},
  macros::sizeudata::sizeudata,
  records::{gc_object::GCObject, udata::Udata},
};

pub(crate) unsafe fn dumpudata(f: *mut c_void, u: *mut Udata) {
  unsafe {
    let u = &*u;
    c_file_write(
      f,
      format_args!(
        "{{\"type\":\"userdata\",\"cat\":{},\"size\":{},\"tag\":{}",
        u.memcat,
        sizeudata(u.len as usize) as c_int,
        u.tag as c_int
      ),
    );

    if !u.metatable.is_null() {
      c_file_write_bytes(f, b",\"metatable\":");
      let mt = u.metatable as *mut GCObject;
      dumpref(f, mt);
    }

    c_file_write_bytes(f, b"}");
  }
}
