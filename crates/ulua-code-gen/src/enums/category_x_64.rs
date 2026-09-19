#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CategoryX64 {
  Reg,
  Mem,
  Imm,
}
