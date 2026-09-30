use core::{ffi::c_void, mem::size_of};

use crate::{
  functions::{c_file_write_bytes, dump_json_head, dumpref::dumpref, dumprefs::dumprefs},
  macros::obj_2_gco::obj2gco,
  records::luau_object::LuauObject,
};

/// # Safety
/// `f` 须为有效的输出目标指针（对应 cpp `FILE*`，可写入）；`inst` 须为存活 `LuauObject`，
/// 其 `lclass` 与 `members[0..numberofmembers]` 须均为存活 GCObject（`dumpref`/`dumprefs` 会递归取其地址）。
/// cpp `lgcdebug.cpp:643`。
pub(crate) unsafe fn dumpobject(f: *mut c_void, inst: &LuauObject) {
  unsafe {
    dump_json_head(f, "object", inst.memcat, size_of::<LuauObject>() as i32);

    c_file_write_bytes(f, b",\"class\":");
    dumpref(f, obj2gco!(inst.lclass));

    c_file_write_bytes(f, b",\"members\":");
    dumprefs(f, inst.members, inst.numberofmembers as usize);

    c_file_write_bytes(f, b"]}");
  }
}
