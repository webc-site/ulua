use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_function::IrFunction, ir_inst::IrInst},
};

impl IrFunction {
  pub fn has_restore_location_ir_inst_bool(
    &self,
    inst: &IrInst,
    limit_to_current_block: bool,
  ) -> bool {
    let restore_location = self.find_restore_location_ir_inst_bool(inst, limit_to_current_block);
    restore_location.op.kind() != IrOpKind::None
  }
}
