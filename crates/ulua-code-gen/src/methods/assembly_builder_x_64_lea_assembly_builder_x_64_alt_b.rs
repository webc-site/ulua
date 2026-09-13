use crate::{
  enums::size_x_64::SizeX64,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, label::Label, operand_x_64::OperandX64,
    register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn lea_register_x_64_label(&mut self, lhs: RegisterX64, label: &mut Label) {
    if !(lhs.size() == SizeX64::Qword) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    let rhs = OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      RegisterX64::RIP,
      0,
    );

    self.place_binary_reg_and_reg_mem(OperandX64::operand_x_64_register_x_64(lhs), rhs, 0x8d, 0x8d);

    self.code_pos = self.code_pos.wrapping_sub(4);

    self.place_label(label);
    self.commit();

    if self.log_text {
      self.log_c_char_register_x_64_label("lea", lhs, *label);
    }
  }
}
