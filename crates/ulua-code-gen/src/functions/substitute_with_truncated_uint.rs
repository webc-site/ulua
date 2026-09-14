use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    produces_dirty_high_register_bits::produces_dirty_high_register_bits,
    replace_ir_utils_alt_b::replace_ir_function_ir_block_u32_ir_inst, substitute::substitute,
  },
  records::{ir_block::IrBlock, ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp},
  type_aliases::ir_ops::IrOps,
};

pub fn substitute_with_truncated_uint(
  function: &mut IrFunction,
  block: &mut IrBlock,
  inst: &mut IrInst,
  op: IrOp,
) {
  let src_of_src: *mut IrInst = function.as_inst_op(op);
  if !src_of_src.is_null() && produces_dirty_high_register_bits(unsafe { (*src_of_src).cmd }) {
    let inst_index = function.get_inst_index(inst);
    let mut ops = IrOps::new();
    ops.push(op);
    let replacement = IrInst {
      cmd: IrCmd::TruncateUint,
      ops,
      ..Default::default()
    };
    replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, replacement);
  } else {
    substitute(function, inst, op);
  }
}
