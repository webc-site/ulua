use crate::{
  enums::category_x_64::CategoryX64,
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};

impl AssemblyBuilderX64 {
  pub fn place_reg_and_mod_reg_mem(
    &mut self,
    lhs: OperandX64,
    rhs: OperandX64,
    extra_code_bytes: i32,
  ) {
    // Avoid CODEGEN_ASSERT!'s pointer-signature mismatch by using an explicit boolean check.
    if !(lhs.cat == CategoryX64::Reg) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    // C++ passes `lhs.base.index` (the register index 0-15). The `{size,index}`
    // bit-packing means raw `.bits` is `index<<3 | size`, so reading it directly
    // fed the SIZE into the ModRM reg field. Use the decoded index.
    self.place_mod_reg_mem(rhs, lhs.base.index(), extra_code_bytes);
  }
}
