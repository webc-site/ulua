use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};
impl AssemblyBuilderA64 {
  pub fn log_c_char_register_a_64_register_a_64_register_a_64_i32(
    &mut self,
    opcode: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    shift: i32,
  ) {
    self.log_append(format_args!(" {:<12}", opcode));

    if dst != RegisterA64::XZR && dst != RegisterA64::WZR {
      self.log_register_a_64(dst);
      self.text.push(',');
    }

    self.log_register_a_64(src1);
    self.text.push(',');
    self.log_register_a_64(src2);

    if src1.kind() == KindA64::X && src2.kind() == KindA64::W {
      self.log_append(format_args!(" UXTW #{}", shift));
    } else if shift > 0 {
      self.log_append(format_args!(" LSL #{}", shift));
    } else if shift < 0 {
      self.log_append(format_args!(" LSR #{}", -shift));
    }

    self.text.push('\n');
  }
}
