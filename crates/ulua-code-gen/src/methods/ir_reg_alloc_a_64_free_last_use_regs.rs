use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_inst::IrInst, ir_reg_alloc_a_64::IrRegAllocA64},
};

impl IrRegAllocA64 {
  pub fn free_last_use_regs(&mut self, inst: &IrInst, index: u32) {
    // Safety: self.function is a valid pointer to an IrFunction, and inst.ops is a valid array
    // of IrOp entries. We assume the caller has ensured the indices are valid as in C++.
    let function = unsafe { &mut *self.function };

    for &op in inst.ops.iter() {
      if op.kind() == IrOpKind::Inst {
        let op_index = op.index();
        let target = &mut function.instructions[op_index as usize];
        self.free_last_use_reg(target, index);
      }
    }
  }
}
