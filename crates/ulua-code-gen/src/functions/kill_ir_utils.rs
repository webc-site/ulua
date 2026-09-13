use crate::{
  enums::ir_cmd::IrCmd,
  functions::remove_use::remove_use,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_function::IrFunction, ir_inst::IrInst},
};

pub fn kill_ir_function_ir_inst(function: &mut IrFunction, inst: &mut IrInst) {
  CODEGEN_ASSERT!(inst.use_count == 0);

  inst.cmd = IrCmd::NOP;

  for op in inst.ops.as_slice() {
    remove_use(function, *op);
  }
  inst.ops.clear();
}
