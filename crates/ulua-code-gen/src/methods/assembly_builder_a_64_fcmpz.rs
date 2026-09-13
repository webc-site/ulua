use crate::{
  enums::kind_a_64::KindA64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn fcmpz(&mut self, src: RegisterA64) {
    CODEGEN_ASSERT!(src.kind() == KindA64::D || src.kind() == KindA64::S);

    let zero_reg = RegisterA64 {
      bits: src.kind() as u8,
    };

    if src.kind() == KindA64::D {
      self.assembly_builder_a_64_place_fcmp("fcmp", src, zero_reg, 0b1111_0011, 0b01);
    } else {
      self.assembly_builder_a_64_place_fcmp("fcmp", src, zero_reg, 0b1111_0001, 0b01);
    }
  }
}
