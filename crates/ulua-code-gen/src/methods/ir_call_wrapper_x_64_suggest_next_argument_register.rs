use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{
    ir_call_wrapper_x_64::IrCallWrapperX64, ir_data::K_INVALID_INST_IDX, register_x_64::RegisterX64,
  },
};

impl IrCallWrapperX64 {
  pub fn suggest_next_argument_register(&self, size: SizeX64) -> RegisterX64 {
    let target = self.get_next_argument_target(size);
    let regs = unsafe { &mut *self.regs };

    if target.cat != CategoryX64::Reg {
      return regs.alloc_reg(size, K_INVALID_INST_IDX);
    }

    if !regs.can_take_reg(target.base) {
      return regs.alloc_reg(size, K_INVALID_INST_IDX);
    }

    regs.take_reg(target.base, K_INVALID_INST_IDX)
  }
}
