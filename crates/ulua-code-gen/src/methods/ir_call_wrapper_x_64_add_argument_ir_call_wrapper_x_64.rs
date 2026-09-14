use crate::{
  enums::{abix_64::ABIX64, ir_op_kind::IrOpKind, size_x_64::SizeX64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    call_argument::CallArgument, ir_call_wrapper_x_64::IrCallWrapperX64,
    ir_data::K_INVALID_INST_IDX, ir_op::IrOp, operand_x_64::OperandX64,
  },
};

impl IrCallWrapperX64 {
  pub fn add_argument_size_x_64_operand_x_64_ir_op(
    &mut self,
    target_size: SizeX64,
    source: OperandX64,
    source_op: IrOp,
  ) {
    // Instruction operands rely on current instruction index for lifetime tracking
    CODEGEN_ASSERT!(self.inst_idx != K_INVALID_INST_IDX || source_op.kind() == IrOpKind::None);

    CODEGEN_ASSERT!(self.arg_count < Self::K_MAX_CALL_ARGUMENTS);

    let target = self.get_next_argument_target(target_size);

    let idx = self.arg_count as usize;
    self.arg_count += 1;

    self.args[idx] = CallArgument {
      target_size,
      source,
      source_op,
      target,
      candidate: true,
    };

    if unsafe { (*self.build).abi } == ABIX64::WINDOWS {
      // On Windows, gpr/xmm register positions move in sync
      self.gpr_pos += 1;
      self.xmm_pos += 1;
    } else if target_size == SizeX64::Xmmword {
      self.xmm_pos += 1;
    } else {
      self.gpr_pos += 1;
    }
  }
}
