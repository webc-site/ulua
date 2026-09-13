use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};

impl AssemblyBuilderX64 {
  pub fn vmovq(&mut self, dst: OperandX64, src: OperandX64) {
    if dst.base.size() == SizeX64::Xmmword {
      if !(dst.cat == CategoryX64::Reg) {
        ulua_common::LUAU_DEBUGBREAK!();
      }
      if !(src.base.size() == SizeX64::Qword) {
        ulua_common::LUAU_DEBUGBREAK!();
      }
      self.place_avx_c_char_operand_x_64_operand_x_64_u8_bool_u8_u8(
        "vmovq", dst, src, 0x6e, true, 0b0001, 0b01,
      );
    } else if dst.base.size() == SizeX64::Qword {
      if !(src.cat == CategoryX64::Reg) {
        ulua_common::LUAU_DEBUGBREAK!();
      }
      if !(src.base.size() == SizeX64::Xmmword) {
        ulua_common::LUAU_DEBUGBREAK!();
      }
      self.place_avx_c_char_operand_x_64_operand_x_64_u8_bool_u8_u8(
        "vmovq", src, dst, 0x7e, true, 0b0001, 0b01,
      );
    } else {
      ulua_common::LUAU_DEBUGBREAK!();
    }
  }
}
