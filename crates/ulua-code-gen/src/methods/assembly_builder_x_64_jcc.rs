use crate::{
  enums::condition_x_64::ConditionX64,
  records::{assembly_builder_x_64::AssemblyBuilderX64, label::Label},
};

impl AssemblyBuilderX64 {
  pub fn jcc(&mut self, cond: ConditionX64, label: &mut Label) {
    let cc = match cond {
      ConditionX64::Overflow => 0x70,
      ConditionX64::NoOverflow => 0x71,
      ConditionX64::Carry => 0x72,
      ConditionX64::NoCarry => 0x73,
      ConditionX64::Below => 0x72,
      ConditionX64::BelowEqual => 0x76,
      ConditionX64::Above => 0x77,
      ConditionX64::AboveEqual => 0x73,
      ConditionX64::Equal => 0x74,
      ConditionX64::Less => 0x7c,
      ConditionX64::LessEqual => 0x7e,
      ConditionX64::Greater => 0x7f,
      ConditionX64::GreaterEqual => 0x7d,
      ConditionX64::NotBelow => 0x73,
      ConditionX64::NotBelowEqual => 0x77,
      ConditionX64::NotAbove => 0x76,
      ConditionX64::NotAboveEqual => 0x72,
      ConditionX64::NotEqual => 0x75,
      ConditionX64::NotLess => 0x7d,
      ConditionX64::NotLessEqual => 0x7f,
      ConditionX64::NotGreater => 0x7e,
      ConditionX64::NotGreaterEqual => 0x7c,
      ConditionX64::Zero => 0x74,
      ConditionX64::NotZero => 0x75,
      ConditionX64::Parity => 0x7a,
      ConditionX64::NotParity => 0x7b,
      ConditionX64::Count => 0x70,
    };
    // C++ passes `jccTextForCondition[cond]` as the disassembler mnemonic — NOT
    // null, which would `strlen(NULL)` in the log path. Order matches the
    // `ConditionX64` discriminants (same order as `codeForCondition`).
    let jcc_text = [
      "jo", "jno", "jc", "jnc", "jb", "jbe", "ja", "jae", "je", "jl", "jle", "jg", "jge", "jnb",
      "jnbe", "jna", "jnae", "jne", "jnl", "jnle", "jng", "jnge", "jz", "jnz", "jp", "jnp",
    ];
    let name = jcc_text[cond as usize];

    // C++ passes `codeForCondition[cond]` (the 0-15 condition code); this match
    // holds the full rel8 opcodes (0x70|cc), so `place_jcc` (which does
    // `0x80 | code`) needs just the low-nibble condition code.
    self.place_jcc(name, label, (cc & 0x0F) as u8);
  }
}
