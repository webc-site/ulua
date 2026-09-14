//! Source: `Analysis/src/Clone.cpp:541-544`
//! `void FragmentAutocompleteTypeCloner::cloneChildren(LazyType* t) override`.
//!
//! Overrides `cloneChildren(LazyType*)` to a no-op: lazy types are not cloned.
//! When the base `run`/`cloneChildren` machinery dispatches a `LazyType`, the
//! base `clone_children_lazy_type` honours `base.skip_lazy_type_clone` (set true
//! by the constructor), so the recursion also skips lazy types faithfully.

use crate::records::{
  fragment_autocomplete_type_cloner::FragmentAutocompleteTypeCloner, lazy_type::LazyType,
};

impl FragmentAutocompleteTypeCloner {
  pub fn clone_children(&mut self, _t: *mut LazyType) {
    // Do not clone lazy types
  }
}
