use crate::{
  enums::size_x_64::SizeX64,
  records::{ir_call_wrapper_x_64::IrCallWrapperX64, register_x_64::RegisterX64},
};

impl IrCallWrapperX64 {
  pub fn get_register_uses(&self, reg: RegisterX64) -> i32 {
    if reg.size() == SizeX64::Xmmword {
      self.xmm_uses[reg.index() as usize] as i32
    } else if reg.size() != SizeX64::None {
      self.gpr_uses[reg.index() as usize] as i32
    } else {
      0
    }
  }
}
