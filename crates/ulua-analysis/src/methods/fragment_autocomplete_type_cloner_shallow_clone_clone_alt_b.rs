//! Source: `Analysis/src/Clone.cpp:520-539`
//! `TypePackId FragmentAutocompleteTypeCloner::shallowClone(TypePackId tp) override`.
//!
//! Free type packs receive `replacementForNullScope` instead of null
//! (Clone.cpp:533-534); that substitution is carried by
//! `base.replacement_for_null_scope`, so this is a faithful delegation.

use crate::{
  records::fragment_autocomplete_type_cloner::FragmentAutocompleteTypeCloner,
  type_aliases::type_pack_id::TypePackId,
};

impl FragmentAutocompleteTypeCloner {
  pub fn shallow_clone_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    self.base.shallow_clone_type_pack_id(tp)
  }
}
