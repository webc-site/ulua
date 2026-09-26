use core::ffi::c_void;

use crate::{
  functions::{
    c_file_write_bytes, c_file_write_str, dump_json_head, dumpref::dumpref, dumprefs::dumprefs,
  },
  macros::{
    getstr::getstr, obj_2_gco::obj2gco, size_cclosure::size_cclosure, size_lclosure::size_lclosure,
  },
  records::closure::Closure,
};

/// # Safety
/// `f` 须为 `c_file_write*` 可写入的有效 FILE 指针；`cl` 须指向存活 Closure，其 `hdr.memcat`/`is_c`/`nupvalues`
/// 自洽：C 闭包时 `inner.c.upvals` 覆盖 `nupvalues` 项、Lua 闭包时 `inner.l.p` 及其 `uprefs` 覆盖 `nupvalues` 项、
/// `(*p).debugname`/`(*p).source` 可读。只读遍历，不改动 GC 状态。cpp/VM/src/lgcdebug.cpp:415 dumpclosure。
pub(crate) unsafe fn dumpclosure(f: *mut c_void, cl: *mut Closure) {
  unsafe {
    dump_json_head(
      f,
      "function",
      (*cl).hdr.memcat,
      if (*cl).is_c != 0 {
        size_cclosure((*cl).nupvalues as i32) as i32
      } else {
        size_lclosure((*cl).nupvalues as usize) as i32
      },
    );

    c_file_write_bytes(f, b",\"env\":");
    dumpref(f, obj2gco!((*cl).env));

    if (*cl).is_c != 0 {
      let c = &(*cl).inner.c;
      if !c.debugname.is_null() {
        c_file_write_bytes(f, b",\"name\":\"");
        c_file_write_str(f, c.debugname);
        c_file_write_bytes(f, b"\"");
      }
      if (*cl).nupvalues != 0 {
        c_file_write_bytes(f, b",\"upvalues\":[");
        dumprefs(f, c.upvals.as_ptr() as *mut _, (*cl).nupvalues as usize);
        c_file_write_bytes(f, b"]");
      }
    } else {
      let l = &(*cl).inner.l;
      if !(*l.p).debugname.is_null() {
        c_file_write_bytes(f, b",\"name\":\"");
        c_file_write_str(f, getstr((*l.p).debugname));
        c_file_write_bytes(f, b"\"");
      }
      c_file_write_bytes(f, b",\"proto\":");
      dumpref(f, obj2gco!(l.p));
      if (*cl).nupvalues != 0 {
        c_file_write_bytes(f, b",\"upvalues\":[");
        dumprefs(f, l.uprefs.as_ptr() as *mut _, (*cl).nupvalues as usize);
        c_file_write_bytes(f, b"]");
      }
    }

    c_file_write_bytes(f, b"}");
  }
}
