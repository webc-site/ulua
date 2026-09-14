use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp},
};

impl IrFunction {
  pub fn inst_op(&mut self, op: IrOp) -> &mut IrInst {
    assert!(op.kind() == IrOpKind::Inst);
    &mut self.instructions[op.index() as usize]
  }
}
