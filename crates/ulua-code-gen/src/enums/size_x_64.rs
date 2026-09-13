#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SizeX64 {
  None,
  Byte,
  Word,
  Dword,
  Qword,
  Xmmword,
  Ymmword,
}
