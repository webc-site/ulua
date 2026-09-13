use crate::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

impl IrBuilder {
  pub fn inst_ir_cmd_ir_op(&mut self, cmd: IrCmd, a: IrOp) -> IrOp {
    // C++ `inst(cmd, {a})` — a size-1 operand list. The original port padded
    // it to two operands with `undef`, leaving a spurious `undef` operand
    // (e.g. `RETURN 0u, undef`).
    self.inst_ir_cmd_initializer_list_ir_op(cmd, &[a])
  }
}
