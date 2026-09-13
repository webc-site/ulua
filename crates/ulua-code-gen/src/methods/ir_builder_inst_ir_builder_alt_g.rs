use crate::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

impl IrBuilder {
  pub fn inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
    &mut self,
    cmd: IrCmd,
    a: IrOp,
    b: IrOp,
    c: IrOp,
    d: IrOp,
    e: IrOp,
    f: IrOp,
  ) -> IrOp {
    self.inst_ir_cmd_initializer_list_ir_op(cmd, &[a, b, c, d, e, f])
  }
}
