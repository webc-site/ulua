use strum::{Display, FromRepr, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromRepr, IntoStaticStr, Display)]
#[strum(serialize_all = "snake_case")]
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
