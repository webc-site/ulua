use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  functions::fresh_index::fresh_index,
  records::{scope::Scope, type_level::TypeLevel},
};
#[derive(Debug, Clone)]
pub struct FreeTypePack {
  pub(crate) index: i32,
  pub(crate) level: TypeLevel,
  pub(crate) scope: *mut Scope,
  pub(crate) polarity: Polarity,
}

impl FreeTypePack {
  pub fn new(level: TypeLevel) -> Self {
    Self {
      index: fresh_index(),
      level,
      scope: null_mut(),
      polarity: Polarity::Unknown,
    }
  }
}
