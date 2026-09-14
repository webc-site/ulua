use core::{
  ffi::{c_char, c_int, c_void},
  mem::size_of,
};

use crate::{
  functions::{c_slice, dumpref::dumpref, dumprefs::dumprefs, dumpstringdata::dumpstringdata},
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

    unsafe extern "C" {
      fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    }

    fprintf(
      f,
      c"{\"type\":\"proto\",\"cat\":%d,\"size\":%d".as_ptr() as *const c_char,
      p.hdr.memcat as c_int,
      size as c_int,
    );

    if !p.source.is_null() {
      fprintf(f, c",\"source\":\"".as_ptr() as *const c_char);
      dumpstringdata(f, (*p.source).data.as_ptr(), (*p.source).len as usize);
      fprintf(
        f,
        c"\",\"line\":%d".as_ptr() as *const c_char,
        if !p.abslineinfo.is_null() {
          *p.abslineinfo
        } else {
          0
        },
      );
    }

    if p.sizek > 0 {
      fprintf(f, c",\"constants\":[".as_ptr() as *const c_char);
      dumprefs(f, p.k, p.sizek as usize);
      fprintf(f, c"]".as_ptr() as *const c_char);
    }

    if p.sizep > 0 {
      unsafe extern "C" {
        fn fputc(c: c_int, stream: *mut c_void) -> c_int;
      }

      fprintf(f, c",\"protos\":[".as_ptr() as *const c_char);
      for (i, &proto_ptr) in c_slice(p.p, p.sizep as usize).iter().enumerate() {
        if i != 0 {
          fputc(',' as c_int, f);
        }
        dumpref(f, &mut (*proto_ptr).hdr as *mut GCheader as *mut GCObject);
      }
      fprintf(f, c"]".as_ptr() as *const c_char);
    }

    fprintf(f, c"}".as_ptr() as *const c_char);
  }
}
