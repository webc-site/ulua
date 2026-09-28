use core::ffi::c_void;

use crate::{
  functions::{c_file_write_bytes, dump_json_head},
  macros::sizebuffer::sizebuffer,
  records::luau_buffer::LuauBuffer as Buffer,
};

/// # Safety
/// `f` 须为可写的合法 `FILE*`（`c_file_write` 目标流）；`b` 须指向存活 `LuauBuffer`，仅读其 `memcat`、`len`
/// 字段（`sizebuffer(b.len)` 求记账大小）；不回收、不改对象、不抛 Lua 错误。仅供 `luaC_freeallobjects` 调试导出使用。
/// cpp VM/src/lgcdebug.cpp:560
pub(crate) unsafe fn dumpbuffer(f: *mut c_void, b: *mut Buffer) {
  unsafe {
    let b = &*b;
    dump_json_head(f, "buffer", b.memcat, sizebuffer(b.len as usize) as i32);
    c_file_write_bytes(f, b"}}");
  }
}
