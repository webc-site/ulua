use core::{ffi::c_void, mem::size_of};

use crate::{
  functions::{c_file_write, c_file_write_bytes, dump_json_head, dumpref::dumpref},
  macros::{gcvalue::gcvalue, iscollectable::iscollectable, upisopen::upisopen},
  records::up_val::UpVal,
};

/// # Safety
/// `f` 须为可写合法 `FILE*`；`uv` 须指向存活 `UpVal`：`upisopen!(uv)` 读其 `u.open/tv` 判定开/闭，
/// 仅当 `uv.v` 为 collectable 时对 `gcvalue!(uv.v)` 递归 `dumpref`（该 GCObject 须存活）；只读导出，不改对象。
/// cpp VM/src/lgcdebug.cpp:601
pub(crate) unsafe fn dumpupval(f: *mut c_void, uv: *mut UpVal) {
  unsafe {
    let uv_ref = &*uv;

    let is_open = upisopen!(uv);

    dump_json_head(f, "upvalue", uv_ref.hdr.memcat, size_of::<UpVal>() as i32);
    c_file_write(
      f,
      format_args!(",\"open\":{}", if is_open { "true" } else { "false" }),
    );

    if iscollectable!(uv_ref.v) {
      c_file_write_bytes(f, b",\"object\":");
      dumpref(f, gcvalue!(uv_ref.v));
    }

    c_file_write_bytes(f, b"}");
  }
}
