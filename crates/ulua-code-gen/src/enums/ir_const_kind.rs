#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IrConstKind {
  Int,
  Int64,
  Uint,
  Double,
  Tag,
  Import,
}
