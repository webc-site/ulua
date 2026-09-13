use crate::{
  macros::codegen_assert::CODEGEN_ASSERT, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn after_inst_for_n_loop(build: &mut IrBuilder, _pc: *const Instruction) {
  CODEGEN_ASSERT!(!build.numeric_loop_stack.is_empty());
  build.numeric_loop_stack.pop();
}
