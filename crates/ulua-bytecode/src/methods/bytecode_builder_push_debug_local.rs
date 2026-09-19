use crate::records::{
  bytecode_builder::BytecodeBuilder, debug_local_bytecode_builder::DebugLocal,
  string_ref::StringRef,
};

impl BytecodeBuilder {
  pub fn push_debug_local(&mut self, name: StringRef, reg: u8, startpc: u32, endpc: u32) {
    let index = self.add_string_table_entry(name);

    let local = DebugLocal {
      name: index,
      reg,
      startpc,
      endpc,
    };

    self.debug_locals.push(local);
  }
}
