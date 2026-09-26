#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Kind {
  IMM26,
  IMM19,
  IMM14,
}

impl Kind {
  pub const IMM26: Kind = Kind::IMM26;
  pub const IMM19: Kind = Kind::IMM19;
  pub const IMM14: Kind = Kind::IMM14;
}
