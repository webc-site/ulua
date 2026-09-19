use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{enums::dump_flags::DumpFlags, records::bytecode_builder::BytecodeBuilder};

impl BytecodeBuilder {
  pub fn annotate_instruction(&self, result: &mut String, fid: u32, instpos: u32) {
    if (self.dump_flags & DumpFlags::Code as u32) == 0 {
      return;
    }

    LUAU_ASSERT!(fid < self.functions.len() as u32);

    let function = &self.functions[fid as usize];
    let (dump, dumpinstoffs) = (&function.dump, &function.dumpinstoffs);

    let next = instpos + 1;

    LUAU_ASSERT!(next < dumpinstoffs.len() as u32);

    // Skip locations of multi-dword instructions（定位下一个非 -1 偏移）
    let next = dumpinstoffs[instpos as usize + 1..]
      .iter()
      .position(|&off| off != -1)
      .map_or(dumpinstoffs.len() as u32, |k| instpos + 1 + k as u32);

    let start_offset = dumpinstoffs[instpos as usize] as usize;
    let end_offset = dumpinstoffs[next as usize] as usize;

    // cpp `formatAppend(result, "%.*s", len, dump.data() + start)`：定长字节追加，
    // Rust 直接 push_str，免过格式化机
    result.push_str(&dump[start_offset..end_offset]);
  }
}
