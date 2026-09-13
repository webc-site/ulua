use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    add_use::add_use, get_op_ir_data::get_op_mut, is_block_terminator::is_block_terminator,
    remove_use::remove_use,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp},
};

pub fn substitute(function: &mut IrFunction, inst: &mut IrInst, replacement: IrOp) {
  CODEGEN_ASSERT!(!is_block_terminator(inst.cmd));

  inst.cmd = IrCmd::SUBSTITUTE;

  add_use(function, replacement);

  let n = inst.ops.size();
  for i in 0..n {
    let op: IrOp = inst.ops.as_slice()[i as usize];
    remove_use(function, op);
  }

  inst.ops.resize(1);
  *get_op_mut(inst, 0) = replacement;
}
