use ulua_common::FFlag;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_op_kind::IrOpKind},
  functions::{emit_abort::emit_abort, vm_exit_op::vm_exit_op},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    exit_handler_ir_lowering_a_64::ExitHandler, ir_lowering_a_64::IrLoweringA64, ir_op::IrOp,
    label::Label,
  },
};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_finalize_target_label(
    &mut self,
    op: IrOp,
    index: u32,
    fresh: &mut Label,
  ) {
    if op.kind() == IrOpKind::Undef {
      unsafe {
        emit_abort(&mut *self.build, fresh);
      }
    } else if FFlag::LuauCodegenVmExitSync.get()
      && op.kind() == IrOpKind::Block
      && unsafe { (*self.ir_lowering_a_64_block_op(op)).kind } == IrBlockKind::ExitSync
    {
      if self.exit_sync_inst_idx == index {
        CODEGEN_ASSERT!(self.exit_sync_alloc_token == self.regs.get_alloc_token());
      }

      let sync_info = unsafe { (*self.function).vm_exit_info.find(&index) };
      CODEGEN_ASSERT!(sync_info.is_some());
      let sync_info = sync_info.unwrap();

      for arg_op in &sync_info.arg_ops {
        let inst_op = unsafe { (*self.function).inst_op(*arg_op) };
        self
          .regs
          .record_and_free_last_use(op.index(), inst_op, index);
      }
    } else if op.kind() == IrOpKind::VmExit && fresh.id != 0 {
      let exit_handler_idx = self.exit_handlers.len() as u32;
      self
        .exit_handler_map
        .try_insert(vm_exit_op(op), exit_handler_idx);
      self.exit_handlers.push(ExitHandler {
        self_: *fresh,
        pcpos: vm_exit_op(op),
      });
    }
  }
}
