use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::ir_builder::IrBuilder};

pub fn after_inst_for_n_loop(build: &mut IrBuilder) {
  CODEGEN_ASSERT!(!build.numeric_loop_stack.is_empty());
  build.numeric_loop_stack.pop();
}
