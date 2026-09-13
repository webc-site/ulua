use core::{
  ffi::c_int,
  fmt::{Debug, Formatter, Result},
};

use crate::type_aliases::value::Value;
#[derive(Clone, Copy)]
#[repr(C)]
#[derive(Default)]
pub struct lua_TValue {
  pub value: Value,
  pub extra: [c_int; 1],
  pub tt: c_int,
}

pub type TValue = lua_TValue;

impl Debug for lua_TValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("lua_TValue")
      .field("extra", &self.extra)
      .field("tt", &self.tt)
      .finish_non_exhaustive()
  }
}

impl lua_TValue {
  /// Tag accessor mirroring `TKey::tt()` so the C++ duck-typed tag macros
  /// (`ttype!`, `setttype!`, `iscollectable!`) work on values AND keys.
  #[inline]
  pub fn tt(&self) -> c_int {
    self.tt
  }

  #[inline]
  pub fn set_tt(&mut self, tt: c_int) {
    self.tt = tt;
  }
}
