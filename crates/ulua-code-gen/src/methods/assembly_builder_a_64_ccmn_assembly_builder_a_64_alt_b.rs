use crate::{
  enums::{condition_a_64::ConditionA64, kind_a_64::KindA64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};
impl AssemblyBuilderA64 {
  pub fn ccmn_register_a_64_u8_condition_a_64_u8(
    &mut self,
    src1: RegisterA64,
    src2: u8,
    cond: ConditionA64,
    nzcv: u8,
  ) {
    if self.log_text {
      self.log_append(format_args!("{:<12}", "ccmn"));
      self.log_register_a_64(src1);
      self.log_append(format_args!(",#{},#{},", src2, nzcv));
      self.log_append(format_args!(
        "{}",
        match cond {
          ConditionA64::Equal => "eq",
          ConditionA64::NotEqual => "ne",
          ConditionA64::CarrySet => "cs",
          ConditionA64::CarryClear => "cc",
          ConditionA64::Minus => "mi",
          ConditionA64::Plus => "pl",
          ConditionA64::Overflow => "vs",
          ConditionA64::NoOverflow => "vc",
          ConditionA64::UnsignedGreater => "hi",
          ConditionA64::UnsignedLessEqual => "ls",
          ConditionA64::GreaterEqual => "ge",
          ConditionA64::Less => "lt",
          ConditionA64::Greater => "gt",
          ConditionA64::LessEqual => "le",
          ConditionA64::Always => "al",
          _ => "nv",
        }
      ));
    }

    CODEGEN_ASSERT!(src1.kind() == KindA64::W || src1.kind() == KindA64::X);
    CODEGEN_ASSERT!(src2 <= 31);

    let sf: u32 = if src1.kind() == KindA64::X {
      0x8000_0000
    } else {
      0
    };

    let code_for_condition: u32 = match cond {
      ConditionA64::Equal => 0,
      ConditionA64::NotEqual => 1,
      ConditionA64::CarrySet => 2,
      ConditionA64::CarryClear => 3,
      ConditionA64::Minus => 4,
      ConditionA64::Plus => 5,
      ConditionA64::Overflow => 6,
      ConditionA64::NoOverflow => 7,
      ConditionA64::UnsignedGreater => 8,
      ConditionA64::UnsignedLessEqual => 9,
      ConditionA64::GreaterEqual => 10,
      ConditionA64::Less => 11,
      ConditionA64::Greater => 12,
      ConditionA64::LessEqual => 13,
      ConditionA64::Always => 14,
      _ => 15,
    };

    // ccmn: sf 01 11010010 imm5 cond 10 Rn 0 nzcv
    let word = (nzcv & 0x0F) as u32
      | ((src1.index() as u32) << 5)
      | (1 << 11)
      | (code_for_condition << 12)
      | ((src2 as u32) << 16)
      | (0b0111010010u32 << 21)
      | sf;

    self.place(word);
    self.commit();
  }
}
