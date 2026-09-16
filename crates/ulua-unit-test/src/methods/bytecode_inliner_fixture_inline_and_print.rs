use alloc::string::String;

use ulua_bytecode::{
  functions::to_function_bytecode_bytecode_graph::to_function_bytecode_bytecode_builder_comp_time_bc_function,
  records::bytecode_builder::BytecodeBuilder,
};

use crate::records::bytecode_inliner_fixture::BytecodeInlinerFixture;
impl BytecodeInlinerFixture {
  pub fn inline_and_print(&mut self, src: &str, call_idx: u32) -> String {
    let (_inlinee, mut caller) = self
      .compile_and_inline(src, call_idx)
      .expect("expected inline result");

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);
    let result = to_function_bytecode_bytecode_builder_comp_time_bc_function(&mut bcb, &mut caller);
    assert!(!result.is_empty());
    bcb.dump_function(0)
  }
}
