use crate::{
  enums::category_x_64::CategoryX64,
  functions::luau_reg::luau_reg,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

pub fn set_luau_reg(build: &mut AssemblyBuilderX64, tmp: RegisterX64, ri: i32, op: OperandX64) {
  debug_assert!(op.cat == CategoryX64::Mem);

  build.vmovups(OperandX64::reg(tmp), op);
  build.vmovups(luau_reg(ri), OperandX64::reg(tmp));
}
