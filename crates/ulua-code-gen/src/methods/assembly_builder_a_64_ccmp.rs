use crate::{
  enums::{condition_a_64::ConditionA64, kind_a_64::KindA64},
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};
impl AssemblyBuilderA64 {
  pub fn ccmp(&mut self, src1: RegisterA64, src2: RegisterA64, cond: ConditionA64, nzcv: u8) {
    if self.log_text {
      self.log_append(format_args!("{:<12}", "ccmp"));
      self.log_register_a_64(src1);
      self.text.push(',');
      self.log_register_a_64(src2);
      self.log_append(format_args!(",#{},{}", nzcv & 0x0F, cond as u32));
    }

    assert!(src1.kind() == KindA64::W || src1.kind() == KindA64::X);
    assert!(src2.kind() == src1.kind());

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

    let word = (nzcv & 0x0F) as u32
      | ((src1.index() as u32) << 5)
      | (code_for_condition << 12)
      | ((src2.index() as u32) << 16)
      | (0b1111010010u32 << 21)
      | sf;

    self.place(word);
    self.commit();
  }
}
