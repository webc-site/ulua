use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  macros::op_plus_reg::op_plus_reg,
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};

impl AssemblyBuilderX64 {
  pub fn pop(&mut self, op: OperandX64) {
    if self.log_text {
      self.log_c_char_operand_x_64("pop", op);
    }

    if !(op.cat == CategoryX64::Reg && op.base.size() == SizeX64::Qword) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_register_x_64(op.base);
    self.place(op_plus_reg(0x58, op.base.index()));
    self.commit();
  }
}
