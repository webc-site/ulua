use core::fmt::Arguments;

use ulua_common::functions::vformat_append::vformatAppend;

use crate::{enums::dump_flags::DumpFlags, records::bytecode_builder::BytecodeBuilder};

impl BytecodeBuilder {
  pub fn add_debug_remark(&mut self, args: Arguments<'_>) {
    if (self.dump_flags & (DumpFlags::Remarks as u32)) == 0 {
      return;
    }

    let offset = self.debug_remark_buffer.len();

    // C++ `addDebugRemark(const char* format, ...)` printf-formats the whole remark.
    // Rust has no printf, so callers pass the fully-rendered message as a single
    // `format_args!(...)` (the C-style `%d`/`%.2f` become `{}`/`{:.2}`).
    vformatAppend(&mut self.debug_remark_buffer, args);

    // we null-terminate all remarks to avoid storing remark length
    self.debug_remark_buffer.push('\0');

    self
      .debug_remarks
      .push((self.insns.len() as u32, offset as u32));

    let bytes = &self.debug_remark_buffer.as_bytes()[offset..];
    let remark_len = memchr::memchr(0, bytes).unwrap_or(bytes.len()) as i32;

    let remark_str = self.debug_remark_buffer[offset..offset + (remark_len as usize)].to_string();

    self.dump_remarks.push((self.debug_line, remark_str));
  }
}
