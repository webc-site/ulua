use crate::{
  enums::condition_a_64::ConditionA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label},
};

impl AssemblyBuilderA64 {
  pub fn b_condition_a_64_label(&mut self, cond: ConditionA64, label: &mut Label) {
    let name = match cond {
      ConditionA64::Equal => "b.eq",
      ConditionA64::NotEqual => "b.ne",
      ConditionA64::CarrySet => "b.cs",
      ConditionA64::CarryClear => "b.cc",
      ConditionA64::Minus => "b.mi",
      ConditionA64::Plus => "b.pl",
      ConditionA64::Overflow => "b.vs",
      ConditionA64::NoOverflow => "b.vc",
      ConditionA64::UnsignedGreater => "b.hi",
      ConditionA64::UnsignedLessEqual => "b.ls",
      ConditionA64::GreaterEqual => "b.ge",
      ConditionA64::Less => "b.lt",
      ConditionA64::Greater => "b.gt",
      ConditionA64::LessEqual => "b.le",
      ConditionA64::Always => "b",
      ConditionA64::Count => "b",
    };
    let code = match cond {
      ConditionA64::Equal => 0b0_1010_1000_0000_u32,
      ConditionA64::NotEqual => 0b0_1010_1000_0001_u32,
      ConditionA64::CarrySet => 0b0_1010_1000_0010_u32,
      ConditionA64::CarryClear => 0b0_1010_1000_0011_u32,
      ConditionA64::Minus => 0b0_1010_1000_0100_u32,
      ConditionA64::Plus => 0b0_1010_1000_0101_u32,
      ConditionA64::Overflow => 0b0_1010_1000_0110_u32,
      ConditionA64::NoOverflow => 0b0_1010_1000_0111_u32,
      ConditionA64::UnsignedGreater => 0b0_1010_1000_1000_u32,
      ConditionA64::UnsignedLessEqual => 0b0_1010_1000_1001_u32,
      ConditionA64::GreaterEqual => 0b0_1010_1000_1010_u32,
      ConditionA64::Less => 0b0_1010_1000_1011_u32,
      ConditionA64::Greater => 0b0_1010_1000_1100_u32,
      ConditionA64::LessEqual => 0b0_1010_1000_1101_u32,
      ConditionA64::Always => 0b0_1010_1011_1111_u32,
      ConditionA64::Count => 0b0_1010_1011_1111_u32,
    };
    self.place_bc(name, label, 0b0101_0100, (code & 0x3F) as u8);
  }
}
