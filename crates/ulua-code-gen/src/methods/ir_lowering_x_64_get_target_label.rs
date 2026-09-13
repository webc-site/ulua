use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::vm_exit_op::vm_exit_op,
  records::{ir_lowering_x_64::IrLoweringX64, ir_op::IrOp, label::Label},
};

impl IrLoweringX64 {
  pub fn get_target_label<'a>(
    &'a mut self,
    op: IrOp,
    _index: u32,
    fresh: &'a mut Label,
  ) -> &'a mut Label {
    if op.kind() == IrOpKind::Undef {
      return fresh;
    }

    if op.kind() == IrOpKind::VmExit {
      if let Some(index) = self.exit_handler_map.find(&vm_exit_op(op)) {
        return &mut self.exit_handlers[*index as usize].self_;
      }

      return fresh;
    }

    unsafe { &mut *self.label_op(op) }
  }
}
