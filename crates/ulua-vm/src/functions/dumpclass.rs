use core::{
  ffi::{c_int, c_void},
  mem::size_of,
};

use crate::{
  functions::{
    c_file_write, c_file_write_bytes, c_slice, dumpref::dumpref, dumprefs::dumprefs,
    dumpstringdata::dumpstringdata,
  },
  records::{gc_object::GCObject, luau_class::LuauClass},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn dumpclass(f: *mut c_void, lco: *mut LuauClass) {
  unsafe {
    let lco = &*lco;

    c_file_write(
      f,
      format_args!(
        "{{\"type\":\"class\",\"cat\":{},\"size\":{}",
        lco.memcat,
        size_of::<LuauClass>() as c_int
      ),
    );

    c_file_write_bytes(f, b",\"name\":");
    dumpstringdata(f, (*lco.name).data.as_ptr(), (*lco.name).len as usize);

    c_file_write_bytes(f, b",\"super\":");
    if !lco.super_.is_null() {
      dumpref(f, lco.super_ as *mut GCObject);
    } else {
      c_file_write_bytes(f, b"null");
    }

    c_file_write_bytes(f, b",\"membernames\":[");
    for (i, &member) in c_slice(lco.offsettomember, lco.numberofallmembers as usize)
      .iter()
      .enumerate()
    {
      if i != 0 {
        c_file_write_bytes(f, b",");
      }
      dumpref(f, member as *mut GCObject);
    }

    c_file_write_bytes(f, b"],\"staticmembers\":[");
    dumprefs(
      f,
      lco.staticmembers,
      (lco.numberofallmembers - lco.numberofinstancemembers) as usize,
    );

    c_file_write_bytes(f, b"],\"instancemetatable\":");
    if !lco.instancemetatable.is_null() {
      dumpref(f, lco.instancemetatable as *mut GCObject);
    } else {
      c_file_write_bytes(f, b"null");
    }

    c_file_write_bytes(f, b",\"memberstooffset\":");
    dumpref(f, lco.memberstooffset as *mut GCObject);

    c_file_write_bytes(f, b"}");
  }
}
