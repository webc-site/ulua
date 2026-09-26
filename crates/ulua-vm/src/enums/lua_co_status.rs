#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, strum::FromRepr, strum::IntoStaticStr, strum::Display,
)]
#[repr(i32)]
pub enum LuaCoStatus {
  #[strum(serialize = "running")]
  CoRun = 0,
  #[strum(serialize = "suspended")]
  CoSus = 1,
  #[strum(serialize = "normal")]
  CoNor = 2,
  #[strum(serialize = "dead")]
  CoFin = 3,
  #[strum(serialize = "dead")]
  CoErr = 4,
}

impl LuaCoStatus {
  /// `lua_costatus` 返回的 `i32` → 枚举；判别式取自枚举本身，无第二份真相。
  #[inline]
  pub const fn from_c_int(cs: i32) -> Option<Self> {
    Self::from_repr(cs)
  }

  /// `lua_costatus` 返回的 `i32` → 枚举；判别式取自枚举本身，无第二份真相。
  #[inline]
  pub const fn from_i32(cs: i32) -> Option<Self> {
    Self::from_repr(cs)
  }

  /// 协程状态字符串（"running" / "suspended" / "normal" / "dead"）。
  #[inline]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::CoRun => "running",
      Self::CoSus => "suspended",
      Self::CoNor => "normal",
      Self::CoFin | Self::CoErr => "dead",
    }
  }

  /// 协程状态 NUL 结尾字节切片，供压栈或 C 契约 API 使用。
  #[inline]
  pub const fn as_bytes(self) -> &'static [u8] {
    match self {
      Self::CoRun => b"running\0",
      Self::CoSus => b"suspended\0",
      Self::CoNor => b"normal\0",
      Self::CoFin | Self::CoErr => b"dead\0",
    }
  }
}
