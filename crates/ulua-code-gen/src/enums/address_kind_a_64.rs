#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AddressKindA64 {
  Reg,  // Reg + Reg
  Imm,  // Reg + Imm
  Pre,  // Reg + Imm, Reg += Imm
  Post, // Reg, Reg += Imm
}
