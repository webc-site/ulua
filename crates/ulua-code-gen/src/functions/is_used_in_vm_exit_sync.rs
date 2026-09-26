use crate::{
  enums::ir_op_kind::IrOpKind, macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_function::IrFunction,
};

/// cpp `isUsedInVmExitSync` (CodeGen/src/IrAnalysis.cpp:140-153):
/// 判断 `target_inst_idx` 是否被 `inst_idx` 处的 VM exit sync 用作参数
pub fn is_used_in_vm_exit_sync(function: &IrFunction, inst_idx: u32, target_inst_idx: u32) -> bool {
  if let Some(sync_info) = function.vm_exit_info.find(&inst_idx) {
    for arg_op in sync_info.arg_ops.as_slice() {
      CODEGEN_ASSERT!(arg_op.kind() == IrOpKind::Inst);

      if arg_op.index() == target_inst_idx {
        return true;
      }
    }
  }

  false
}
