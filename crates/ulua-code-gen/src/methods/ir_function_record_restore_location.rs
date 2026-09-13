use crate::{
  enums::ir_op_kind::IrOpKind,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_function::IrFunction, value_restore_location::ValueRestoreLocation},
};

impl IrFunction {
  pub fn record_restore_location(&mut self, inst_idx: u32, location: ValueRestoreLocation) {
    CODEGEN_ASSERT!(
      location.op.kind() == IrOpKind::None
        || location.op.kind() == IrOpKind::VmReg
        || location.op.kind() == IrOpKind::VmConst
    );

    if inst_idx >= self.value_restore_ops.len() as u32 {
      self
        .value_restore_ops
        .resize(inst_idx as usize + 1, ValueRestoreLocation::default());
    }

    self.value_restore_ops[inst_idx as usize] = location;
  }
}
