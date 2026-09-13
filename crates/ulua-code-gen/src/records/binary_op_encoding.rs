#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BinaryOpEncoding {
  pub codeimm8: u8,
  pub codeimm: u8,
  pub codeimm_imm8: u8,
  pub code8rev: u8,
  pub coderev: u8,
  pub code8: u8,
  pub code: u8,
  pub opreg: u8,
}

impl BinaryOpEncoding {
  pub const ADD: Self = Self {
    codeimm8: 0x80,
    codeimm: 0x81,
    codeimm_imm8: 0x83,
    code8rev: 0x00,
    coderev: 0x01,
    code8: 0x02,
    code: 0x03,
    opreg: 0,
  };
  pub const OR: Self = Self {
    codeimm8: 0x80,
    codeimm: 0x81,
    codeimm_imm8: 0x83,
    code8rev: 0x08,
    coderev: 0x09,
    code8: 0x0a,
    code: 0x0b,
    opreg: 1,
  };
  pub const AND: Self = Self {
    codeimm8: 0x80,
    codeimm: 0x81,
    codeimm_imm8: 0x83,
    code8rev: 0x20,
    coderev: 0x21,
    code8: 0x22,
    code: 0x23,
    opreg: 4,
  };
  pub const SUB: Self = Self {
    codeimm8: 0x80,
    codeimm: 0x81,
    codeimm_imm8: 0x83,
    code8rev: 0x28,
    coderev: 0x29,
    code8: 0x2a,
    code: 0x2b,
    opreg: 5,
  };
  pub const XOR: Self = Self {
    codeimm8: 0x80,
    codeimm: 0x81,
    codeimm_imm8: 0x83,
    code8rev: 0x30,
    coderev: 0x31,
    code8: 0x32,
    code: 0x33,
    opreg: 6,
  };
  pub const CMP: Self = Self {
    codeimm8: 0x80,
    codeimm: 0x81,
    codeimm_imm8: 0x83,
    code8rev: 0x38,
    coderev: 0x39,
    code8: 0x3a,
    code: 0x3b,
    opreg: 7,
  };
  pub const TEST: Self = Self {
    codeimm8: 0xf6,
    codeimm: 0xf7,
    codeimm_imm8: 0xf7,
    code8rev: 0x84,
    coderev: 0x85,
    code8: 0x84,
    code: 0x85,
    opreg: 0,
  };
}
