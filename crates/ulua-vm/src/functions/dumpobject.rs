use core::{
  ffi::{c_int, c_void},
  mem::size_of,
};

use crate::{
  functions::{c_file_write, c_file_write_bytes, dumpref::dumpref, dumprefs::dumprefs},
  macros::obj_2_gco::obj2gco,
  type_aliases::luau_object::LuauObject,
};

pub(crate) unsafe fn dumpobject(f: *mut c_void, inst: *mut LuauObject) {
  unsafe {
    let inst_ref = &*inst;

    c_file_write(
      f,
      format_args!(
        "{{\"type\":\"object\",\"cat\":{},\"size\":{}",
        inst_ref.memcat,
        size_of::<LuauObject>() as c_int
      ),
    );

    c_file_write_bytes(f, b",\"class\":");
    dumpref(f, obj2gco!(inst_ref.lclass));

    c_file_write_bytes(f, b",\"members\":");
    dumprefs(f, inst_ref.members, inst_ref.numberofmembers as usize);

    c_file_write_bytes(f, b"]}");
  }
}
