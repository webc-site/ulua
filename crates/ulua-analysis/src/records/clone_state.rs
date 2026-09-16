use core::ptr::null;

use crate::{
  records::builtin_types::BuiltinTypes,
  type_aliases::{seen_type_packs_clone::SeenTypePacks, seen_types_clone::SeenTypes},
};
#[derive(Debug)]
pub struct CloneState {
  pub(crate) builtin_types: *mut BuiltinTypes,
  pub(crate) seen_types: SeenTypes,
  pub(crate) seen_type_packs: SeenTypePacks,
}

impl CloneState {
  pub fn new(builtin_types: &mut BuiltinTypes) -> Self {
    Self {
      builtin_types: builtin_types as *mut BuiltinTypes,
      seen_types: SeenTypes::new(null()),
      seen_type_packs: SeenTypePacks::new(null()),
    }
  }
}
