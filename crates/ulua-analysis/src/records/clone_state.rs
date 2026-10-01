use core::ptr::null;

use crate::{
  records::{arena_handle::Handle, builtin_types::BuiltinTypes},
  type_aliases::{seen_type_packs_clone::SeenTypePacks, seen_types_clone::SeenTypes},
};
#[derive(Debug)]
pub struct CloneState {
  pub(crate) builtin_types: Handle<BuiltinTypes>,
  pub(crate) seen_types: SeenTypes,
  pub(crate) seen_type_packs: SeenTypePacks,
}

impl CloneState {
  pub fn new(builtin_types: &mut BuiltinTypes) -> Self {
    Self {
      builtin_types: Handle::from_mut(builtin_types),
      seen_types: SeenTypes::new(null()),
      seen_type_packs: SeenTypePacks::new(null()),
    }
  }
}
