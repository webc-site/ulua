use ulua_common::FFlag;

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_op_kind::IrOpKind},
  functions::vm_exit_op::vm_exit_op,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    exit_handler_ir_lowering_x_64::ExitHandler, ir_lowering_x_64::IrLoweringX64, ir_op::IrOp,
    label::Label,
  },
};

impl IrLoweringX64 {
  pub fn finalize_target_label(&mut self, op: IrOp, index: u32, fresh: &mut Label) {
    if FFlag::LuauCodegenVmExitSync.get()
      && op.kind() == IrOpKind::Block
      && unsafe { (*self.function).block_op(op).kind } == IrBlockKind::ExitSync
    {
      // If branches were emitted via jumpOrAbortOnUndefNoFinalize, verify no allocations happened since
      if self.exit_sync_inst_idx == index {
        CODEGEN_ASSERT!(self.exit_sync_alloc_token == self.regs.get_alloc_token());
      }

      // Snapshot current register/spill locations of values the exit sync block needs, and release registers at last use
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
      *self.exit_handler_map.get_or_insert(vm_exit_op(op)) = exit_handler_idx;
      self.exit_handlers.push(ExitHandler {
        self_: *fresh,
        pcpos: vm_exit_op(op),
      });
    }
  }
}
