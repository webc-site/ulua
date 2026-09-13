#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[derive(Default)]
pub enum Flags {
  #[default]
  NoneSafe = 1 << 0,
}

impl Flags {
  pub const FLAG_NONE_SAFE: Flags = Flags::NoneSafe;
}
