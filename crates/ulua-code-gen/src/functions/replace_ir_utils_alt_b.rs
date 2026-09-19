use core::mem::take;

use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::{
    add_use::add_use, is_block_terminator::is_block_terminator,
    kill_ir_utils_alt_b::kill_ir_function_u32_u32, remove_use::remove_use,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_block::IrBlock, ir_function::IrFunction, ir_inst::IrInst},
};

pub fn replace_ir_function_ir_block_u32_ir_inst(
  function: &mut IrFunction,
  block: &mut IrBlock,
  inst_idx: u32,
  mut replacement: IrInst,
) {
  // Add uses before removing new ones if those are the last ones keeping target operand alive
  for &op in replacement.ops.as_slice() {
    add_use(function, op);
  }

  // If we introduced an earlier terminating instruction, all following instructions become dead
  let inst_cmd = function.instructions[inst_idx as usize].cmd;
  if !is_block_terminator(inst_cmd) && is_block_terminator(replacement.cmd) {
    // Block has has to be fully constructed before replacement is performed
    CODEGEN_ASSERT!(block.finish != !0u32);
    CODEGEN_ASSERT!(inst_idx < block.finish);

    kill_ir_function_u32_u32(function, inst_idx + 1, block.finish);

    // If killing that range killed the current block we have to undo replacement instruction uses and exit
    if block.kind == IrBlockKind::Dead {
      for &op in replacement.ops.as_slice() {
        remove_use(function, op);
      }
      return;
    }

    block.finish = inst_idx;
  }

  let inst = &mut function.instructions[inst_idx as usize];
  // Inherit existing use count (last use is skipped as it will be defined later)
  replacement.use_count = inst.use_count;

  // 取出旧操作数即可，无需整条 IrInst clone
  let old_ops = take(&mut inst.ops);
  *inst = replacement;

  for op in old_ops.as_slice() {
    remove_use(function, *op);
  }
}
