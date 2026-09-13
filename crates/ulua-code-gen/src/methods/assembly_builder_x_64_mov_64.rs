use crate::{
  enums::size_x_64::SizeX64,
  macros::op_plus_reg::op_plus_reg,
  records::{assembly_builder_x_64::AssemblyBuilderX64, register_x_64::RegisterX64},
};

impl AssemblyBuilderX64 {
  pub fn mov64(&mut self, lhs: RegisterX64, imm: i64) {
    if self.log_text {
      self.text.push_str(" mov         ");
      self.log_operand_x_64(lhs.into());
      self.log_append(format_args!(",{:X}h\n", imm as u64));
    }

    if !(lhs.size() == SizeX64::Qword) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_register_x_64(lhs);
    self.place(op_plus_reg(0xb8, lhs.index()));
    self.place_imm_64(imm);
    self.commit();
  }
}
