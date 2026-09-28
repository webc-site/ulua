use core::ffi::c_void;

use crate::{
  functions::{c_file_write_bytes, dump_json_head, dumpstringdata::dumpstringdata},
  macros::sizestring::sizestring,
  records::t_string::tstring,
};

/// # Safety
/// `f` 须为有效输出目标（对应 cpp `FILE*`，可写）；`ts` 须为存活 `tstring`，读其 `hdr.memcat/len`
/// 并把 `data[0..len]` 交给 `dumpstringdata` 逐字节写出（区间长度取自 `ts.len`）。cpp `lgcdebug.cpp:353`。
pub(crate) unsafe fn dumpstring(f: *mut c_void, ts: *mut tstring) {
  unsafe {
    let ts = &*ts;

    dump_json_head(f, "string", ts.hdr.memcat, sizestring(ts.len as usize));
    c_file_write_bytes(f, b",\"data\":\"");

    dumpstringdata(f, ts.data.as_ptr(), ts.len as usize);

    c_file_write_bytes(f, b"\"}\"");
  }
}
