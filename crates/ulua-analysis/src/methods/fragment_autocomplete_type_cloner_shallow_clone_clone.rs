//! Source: `Analysis/src/Clone.cpp:493-518`
//! `TypeId FragmentAutocompleteTypeCloner::shallowClone(TypeId ty) override`.
//!
//! The override differs from the base `shallowClone` only in that free and table
//! types receive `replacementForNullScope` instead of a null scope (Clone.cpp:508-513).
//! In this port that substitution is carried by `base.replacement_for_null_scope`
//! (set by the constructor), so the override is a faithful delegation to the base
//! method, which reads that field.

use crate::{
  records::fragment_autocomplete_type_cloner::FragmentAutocompleteTypeCloner,
  type_aliases::type_id::TypeId,
};

impl FragmentAutocompleteTypeCloner {
  pub fn shallow_clone_type_id(&mut self, ty: TypeId) -> TypeId {
    self.base.shallow_clone_type_id(ty)
  }
}
