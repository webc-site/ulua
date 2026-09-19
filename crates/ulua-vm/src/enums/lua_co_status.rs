use core::ffi::c_int;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum LuaCoStatus {
  CoRun = 0,
  CoSus = 1,
  CoNor = 2,
  CoFin = 3,
  CoErr = 4,
}

impl LuaCoStatus {
  /// `lua_costatus` 返回的 `c_int` → 枚举；判别式取自枚举本身，无第二份真相。
  pub const fn from_c_int(cs: c_int) -> Option<Self> {
    match cs {
      v if v == Self::CoRun as c_int => Some(Self::CoRun),
      v if v == Self::CoSus as c_int => Some(Self::CoSus),
      v if v == Self::CoNor as c_int => Some(Self::CoNor),
      v if v == Self::CoFin as c_int => Some(Self::CoFin),
      v if v == Self::CoErr as c_int => Some(Self::CoErr),
      _ => None,
    }
  }
}
