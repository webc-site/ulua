use crate::{
  enums::{ir_op_kind::IrOpKind, size_x_64::SizeX64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_op::IrOp, ir_reg_alloc_x_64::IrRegAllocX64, register_x_64::RegisterX64},
};

impl IrRegAllocX64 {
  pub fn alloc_reg_or_reuse(
    &mut self,
    size: SizeX64,
    inst_idx: u32,
    oprefs: &[IrOp],
  ) -> RegisterX64 {
    for &op in oprefs {
      if op.kind() != IrOpKind::Inst {
        continue;
      }

      let function = unsafe { &mut *self.function };
      let source = &mut function.instructions[op.index() as usize];

      if source.last_use == inst_idx
        && !source.reused_reg
        && !source.spilled
        && !source.needs_reload
      {
        if (size == SizeX64::Xmmword) != (source.reg_x64.size() == SizeX64::Xmmword) {
          continue;
        }

        CODEGEN_ASSERT!(source.reg_x64.register_x_64_operator_ne(RegisterX64::NOREG));

        source.reused_reg = true;

        if size == SizeX64::Xmmword {
          self.xmm_inst_users[source.reg_x64.index() as usize] = inst_idx;
        } else {
          self.gpr_inst_users[source.reg_x64.index() as usize] = inst_idx;
        }

        return RegisterX64 {
          bits: (size as u8) | (source.reg_x64.index() << RegisterX64::INDEX_SHIFT as u8),
        };
      }
    }

    self.alloc_reg(size, inst_idx)
  }
}
