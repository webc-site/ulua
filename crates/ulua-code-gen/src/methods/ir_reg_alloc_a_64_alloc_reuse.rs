use crate::{
  enums::{ir_op_kind::IrOpKind, kind_a_64::KindA64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_op::IrOp, ir_reg_alloc_a_64::IrRegAllocA64, register_a_64::RegisterA64},
};

impl IrRegAllocA64 {
  pub fn alloc_reuse(&mut self, kind: KindA64, index: u32, oprefs: &[IrOp]) -> RegisterA64 {
    for &op in oprefs {
      if op.kind() != IrOpKind::Inst {
        continue;
      }

      let function = unsafe { &mut *self.function };
      let source = &mut function.instructions[op.index() as usize];

      if source.last_use == index
        && !source.reused_reg
        && source.reg_a64.register_a_64_operator_ne(RegisterA64::NOREG)
      {
        CODEGEN_ASSERT!(true);
        CODEGEN_ASSERT!(true);

        let set = self.get_set(kind);
        CODEGEN_ASSERT!(true);

        set.defs[source.reg_a64.index() as usize] = index;

        source.reused_reg = true;
        return source.reg_a64;
      }
    }

    self.alloc_reg(kind, index)
  }
}
