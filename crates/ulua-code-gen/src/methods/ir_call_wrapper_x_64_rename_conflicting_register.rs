use crate::{
  enums::size_x_64::SizeX64,
  records::{
    ir_call_wrapper_x_64::IrCallWrapperX64, ir_data::K_INVALID_INST_IDX, operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

impl IrCallWrapperX64 {
  pub fn rename_conflicting_register(&mut self, conflict: RegisterX64) {
    // Get a fresh register
    let fresh_reg = unsafe { (*self.regs).alloc_reg(conflict.size(), K_INVALID_INST_IDX) };

    if conflict.size() == SizeX64::Xmmword {
      unsafe {
        (*self.build).vmovsd_operand_x_64_operand_x_64_operand_x_64(
          OperandX64::reg(fresh_reg),
          OperandX64::reg(conflict),
          OperandX64::reg(conflict),
        );
      }
    } else {
      unsafe {
        (*self.build).mov(OperandX64::reg(fresh_reg), OperandX64::reg(conflict));
      }
    }

    self.rename_source_registers(conflict, fresh_reg);
  }
}
