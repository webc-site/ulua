#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BuiltinInfo {
  pub params: i32,
  pub results: i32,
  pub flags: u32,
}

impl BuiltinInfo {
  pub const FLAG_NONE_SAFE: u32 = 1 << 0;
}
