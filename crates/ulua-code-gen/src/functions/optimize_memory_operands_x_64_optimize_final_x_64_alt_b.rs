//! Node: `cxx:Function:Luau.CodeGen:CodeGen/src/OptimizeFinalX64.cpp:134:optimize_memory_operands_x_64`
//!
//! Top-level driver: run the per-block memory-operand optimization over every
//! live block (skipping Dead, and ExitSync when `LuauCodegenVmExitSync` is set).

use ulua_common::FFlag::LuauCodegenVmExitSync;

use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::optimize_memory_operands_x_64_optimize_final_x_64::optimize_memory_operands_x_64_ir_function_ir_block,
  records::{ir_block::IrBlock, ir_function::IrFunction},
};

pub fn optimize_memory_operands_x_64(function: &mut IrFunction) {
  let count = function.blocks.len();
  for i in 0..count {
    let kind = function.blocks[i].kind;

    if kind == IrBlockKind::Dead {
      continue;
    }

    // Inlining a load into its consumer inside the ExitSync block would kill the
    // operands listed in VM exit sync info argOps
    if LuauCodegenVmExitSync.get() && kind == IrBlockKind::ExitSync {
      continue;
    }

    let block: *mut IrBlock = &mut function.blocks[i];
    optimize_memory_operands_x_64_ir_function_ir_block(function, unsafe { &mut *block });
  }
}
