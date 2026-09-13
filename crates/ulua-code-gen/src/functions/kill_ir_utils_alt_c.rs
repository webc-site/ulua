use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::kill_ir_utils_alt_b::kill_ir_function_u32_u32,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_block::IrBlock, ir_function::IrFunction},
};

pub fn kill_ir_function_ir_block(function: &mut IrFunction, block: &mut IrBlock) {
  CODEGEN_ASSERT!(block.use_count == 0);

  block.kind = IrBlockKind::Dead;

  kill_ir_function_u32_u32(function, block.start, block.finish);
  block.start = !0u32;
  block.finish = !0u32;
}
