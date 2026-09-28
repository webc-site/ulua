use core::ptr::{null, null_mut};

use crate::{
  enums::polarity::Polarity,
  records::{scope::Scope, type_level::TypeLevel},
  type_aliases::type_id::TypeId,
};
#[derive(Debug, Clone)]
pub struct FreeType {
  pub index: i32,
  pub level: TypeLevel,
  pub scope: *mut Scope,
  /// True if this free type variable is part of a mutually
  /// recursive type alias whose definitions haven't been
  /// resolved yet.
  pub forwarded_type_alias: bool,
  /// Only used under local type inference
  pub lower_bound: TypeId,
  /// Only used under local type inference
  pub upper_bound: TypeId,
  pub polarity: Polarity,
}

impl Default for FreeType {
  fn default() -> Self {
    Self {
      index: 0,
      level: TypeLevel::default(),
      scope: null_mut(),
      forwarded_type_alias: false,
      lower_bound: null(),
      upper_bound: null(),
      polarity: Polarity::Unknown,
    }
  }
}
