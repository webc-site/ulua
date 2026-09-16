use core::{
  ffi::{c_int, c_void},
  mem::size_of,
};

use crate::{
  functions::{
    c_file_write, c_file_write_bytes, c_slice, dumpref::dumpref, dumprefs::dumprefs,
    dumpstringdata::dumpstringdata,
  },
  records::{g_cheader::GCheader, gc_object::GCObject, t_string::tstring},
  type_aliases::{instruction::Instruction, loc_var::LocVar, proto::Proto, t_value::TValue},
};

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

    c_file_write(
      f,
      format_args!(
        "{{\"type\":\"proto\",\"cat\":{},\"size\":{}",
        p.hdr.memcat, size as c_int
      ),
    );

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
