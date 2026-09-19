use crate::{enums::dump_flags::DumpFlags, records::bytecode_builder::BytecodeBuilder};

impl BytecodeBuilder {
  pub fn needs_debug_remarks(&self) -> bool {
    (self.dump_flags & DumpFlags::Remarks as u32) != 0
  }
}
