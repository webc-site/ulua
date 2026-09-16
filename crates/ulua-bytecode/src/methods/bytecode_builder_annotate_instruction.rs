use ulua_common::{functions::format_append::formatAppend, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::dump_flags::DumpFlags,
  records::{bytecode_builder::BytecodeBuilder, function::Function},
};

impl BytecodeBuilder {
  pub fn annotate_instruction(&self, result: &mut String, fid: u32, instpos: u32) {
    if (self.dump_flags & DumpFlags::Code as u32) == 0 {
      return;
    }

    LUAU_ASSERT!(fid < self.functions.len() as u32);

    let function: &Function = &self.functions[fid as usize];
    let dump: &String = &function.dump;
    let dumpinstoffs: &Vec<i32> = &function.dumpinstoffs;

    let mut next = instpos + 1;

    LUAU_ASSERT!(next < dumpinstoffs.len() as u32);

    // Skip locations of multi-dword instructions
    while next < dumpinstoffs.len() as u32 && dumpinstoffs[next as usize] == -1 {
      next += 1;
    }

    let start_offset = dumpinstoffs[instpos as usize] as usize;
    let end_offset = dumpinstoffs[next as usize] as usize;

    formatAppend(result, format_args!("{}", &dump[start_offset..end_offset]));
  }
}
