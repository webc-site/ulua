use crate::{
  enums::ir_cmd::IrCmd,
  functions::kill_ir_utils::kill_ir_function_ir_inst,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_function::IrFunction, ir_inst::IrInst},
};

pub fn kill_ir_function_u32_u32(function: &mut IrFunction, start: u32, end: u32) {
  // Kill instructions in reverse order to avoid killing instructions that are still marked as used
  let mut i = end as i64;
  while i >= start as i64 {
    CODEGEN_ASSERT!((i as usize) < function.instructions.len());
    let curr: *mut IrInst = &mut function.instructions[i as usize];

    if unsafe { (*curr).cmd } == IrCmd::NOP {
      i -= 1;
      continue;
    }

    // Do not force destruction of instructions that are still in use
    // When the operands are released, the instruction will be released automatically
    if unsafe { (*curr).use_count } != 0 {
      i -= 1;
      continue;
    }

    kill_ir_function_ir_inst(function, unsafe { &mut *curr });
    i -= 1;
  }
}
