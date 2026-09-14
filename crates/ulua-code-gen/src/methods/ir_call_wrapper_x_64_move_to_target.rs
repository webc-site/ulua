use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{call_argument::CallArgument, ir_call_wrapper_x_64::IrCallWrapperX64},
};

impl IrCallWrapperX64 {
  pub fn move_to_target(&mut self, arg: &mut CallArgument) {
    let source_cat = arg.source.cat;
    if source_cat == CategoryX64::Reg {
      let source = arg.source.base;

      if source.size() == SizeX64::Xmmword {
        unsafe {
          (*self.build).vmovsd_operand_x_64_operand_x_64_operand_x_64(
            arg.target,
            source.into(),
            source.into(),
          )
        };
      } else {
        unsafe { (*self.build).mov(arg.target, source.into()) };
      }
    } else if source_cat == CategoryX64::Imm {
      unsafe { (*self.build).mov(arg.target, arg.source) };
    } else {
      if arg.source.mem_size == SizeX64::None {
        unsafe {
          (*self.build).lea_operand_x_64_operand_x_64(arg.target, arg.source);
        };
      } else if arg.target.base.size() == SizeX64::Xmmword
        && arg.source.mem_size == SizeX64::Xmmword
      {
        unsafe { (*self.build).vmovups(arg.target, arg.source) };
      } else if arg.target.base.size() == SizeX64::Xmmword {
        unsafe {
          (*self.build).vmovsd_operand_x_64_operand_x_64(arg.target, arg.source);
        };
      } else {
        unsafe { (*self.build).mov(arg.target, arg.source) };
      }
    }
  }
}
