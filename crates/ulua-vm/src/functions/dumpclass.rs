use core::{
  ffi::{c_char, c_int, c_void},
  mem::size_of,
};

use crate::{
  functions::{c_slice, dumpref::dumpref, dumprefs::dumprefs, dumpstringdata::dumpstringdata},
  records::{gc_object::GCObject, luau_class::LuauClass},
};

pub(crate) unsafe fn dumpclass(f: *mut c_void, lco: *mut LuauClass) {
  unsafe {
    let lco = &*lco;

    unsafe extern "C" {
      fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
      fn fputc(character: c_int, stream: *mut c_void) -> c_int;
    }

    fprintf(
      f,
      c"{\"type\":\"class\",\"cat\":%d,\"size\":%d".as_ptr() as *const c_char,
      lco.memcat as c_int,
      size_of::<LuauClass>() as c_int,
    );

    fprintf(f, c",\"name\":".as_ptr() as *const c_char);
    dumpstringdata(f, (*lco.name).data.as_ptr(), (*lco.name).len as usize);

    fprintf(f, c",\"super\":".as_ptr() as *const c_char);
    if !lco.super_.is_null() {
      dumpref(f, lco.super_ as *mut GCObject);
    } else {
      fprintf(f, c"null".as_ptr() as *const c_char);
    }

    fprintf(f, c",\"membernames\":[".as_ptr() as *const c_char);
    for (i, &member) in c_slice(lco.offsettomember, lco.numberofallmembers as usize)
      .iter()
      .enumerate()
    {
      if i != 0 {
        fputc(',' as c_int, f);
      }
      dumpref(f, member as *mut GCObject);
    }

    fprintf(f, c"],\"staticmembers\":[".as_ptr() as *const c_char);
    dumprefs(
      f,
      lco.staticmembers,
      (lco.numberofallmembers - lco.numberofinstancemembers) as usize,
    );

    fprintf(f, c"],\"instancemetatable\":".as_ptr() as *const c_char);
    if !lco.instancemetatable.is_null() {
      dumpref(f, lco.instancemetatable as *mut GCObject);
    } else {
      fprintf(f, c"null".as_ptr() as *const c_char);
    }

    fprintf(f, c",\"memberstooffset\":".as_ptr() as *const c_char);
    dumpref(f, lco.memberstooffset as *mut GCObject);

    fprintf(f, c"}".as_ptr() as *const c_char);
  }
}
