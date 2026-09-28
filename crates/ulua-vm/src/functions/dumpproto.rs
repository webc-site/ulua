use core::{ffi::c_void, mem::size_of};

use crate::{
  functions::{
    c_file_write, c_file_write_bytes, c_slice, dump_json_head, dumpref::dumpref,
    dumprefs::dumprefs, dumpstringdata::dumpstringdata,
  },
  records::{
    g_cheader::GCheader, gc_object::GCObject, loc_var::LocVar, proto::Proto, t_string::tstring,
  },
  type_aliases::{instruction::Instruction, t_value::TValue},
};

/// # Safety
/// `f` 须为可写合法 `FILE*`；`p` 须指向存活 `Proto`，且其各子数组长度与计数字段自洽：`code` 覆盖
/// `sizecode` 项、`p` 覆盖 `sizep` 项、`k` 覆盖 `sizek` 项、`lineinfo` 覆盖 `sizelineinfo` 字节、`locvars`
/// 覆盖 `sizelocvars` 项、`upvalues` 覆盖 `sizeupvalues` 项；`source` 允许 NULL（非空时其 `data[0..len]` 须可读），
/// `abslineinfo` 允许 NULL（空则回退 0）。只读导出，不改对象、不回收。
/// cpp VM/src/lgcdebug.cpp:565
pub(crate) unsafe fn dumpproto(f: *mut c_void, p: *mut Proto) {
  unsafe {
    let p = &*p;

    let size = size_of::<Proto>()
      + size_of::<Instruction>() * p.sizecode as usize
      + size_of::<*mut Proto>() * p.sizep as usize
      + size_of::<TValue>() * p.sizek as usize
      + p.sizelineinfo as usize
      + size_of::<LocVar>() * p.sizelocvars as usize
      + size_of::<*mut tstring>() * p.sizeupvalues as usize;

    dump_json_head(f, "proto", p.hdr.memcat, size as i32);

    if !p.source.is_null() {
      c_file_write_bytes(f, b",\"source\":\"");
      dumpstringdata(f, (*p.source).data.as_ptr(), (*p.source).len as usize);
      c_file_write(
        f,
        format_args!(
          "\",\"line\":{}",
          if !p.abslineinfo.is_null() {
            *p.abslineinfo
          } else {
            0
          }
        ),
      );
    }

    if p.sizek > 0 {
      c_file_write_bytes(f, b",\"constants\":[");
      dumprefs(f, p.k, p.sizek as usize);
      c_file_write_bytes(f, b"]");
    }

    if p.sizep > 0 {
      c_file_write_bytes(f, b",\"protos\":[");
      for (i, &proto_ptr) in c_slice(p.p, p.sizep as usize).iter().enumerate() {
        if i != 0 {
          c_file_write_bytes(f, b",");
        }
        dumpref(f, &mut (*proto_ptr).hdr as *mut GCheader as *mut GCObject);
      }
      c_file_write_bytes(f, b"]");
    }

    c_file_write_bytes(f, b"}");
  }
}
