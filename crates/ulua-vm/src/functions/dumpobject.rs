use core::{
  ffi::{c_char, c_int, c_void},
  mem::size_of,
};

use crate::{
  functions::{dumpref::dumpref, dumprefs::dumprefs},
  macros::obj_2_gco::obj2gco,
  type_aliases::luau_object::LuauObject,
};

pub(crate) unsafe fn dumpobject(f: *mut c_void, inst: *mut LuauObject) {
  unsafe {
    unsafe extern "C" {
      fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    }

    let inst_ref = &*inst;

    fprintf(
      f,
      c"{\"type\":\"object\",\"cat\":%d,\"size\":%d".as_ptr() as *const c_char,
      inst_ref.memcat as c_int,
      size_of::<LuauObject>() as c_int,
    );

    fprintf(f, c",\"class\":".as_ptr() as *const c_char);
    dumpref(f, obj2gco!(inst_ref.lclass));

    fprintf(f, c",\"members\":".as_ptr() as *const c_char);
    dumprefs(f, inst_ref.members, inst_ref.numberofmembers as usize);

    fprintf(f, c"]}".as_ptr() as *const c_char);
  }
}
