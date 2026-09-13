use crate::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

impl IrBuilder {
  pub fn inst_ir_cmd(&mut self, cmd: IrCmd) -> IrOp {
    // C++ `inst(cmd, {})` — an empty operand list (not a single `undef`).
    self.inst_ir_cmd_initializer_list_ir_op(cmd, &[])
  }
}
