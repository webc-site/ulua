//! Source: `Analysis/src/Clone.cpp:473-545`.
//!
//! `class FragmentAutocompleteTypeCloner final : public TypeCloner`. In C++ the
//! fragment cloner inherits all of `TypeCloner`'s state (`arena`, `builtinTypes`,
//! `types`, `packs`, `queue`, `forceTy`, `forceTp`) and machinery
//! (`clone`/`run`/`find`/`shallowClone`/`cloneChildren`), adding a
//! `Scope* replacementForNullScope` and overriding `shallowClone` and
//! `cloneChildren(LazyType*)`.
//!
//! The Rust port realizes the subclass by *composition*: the base `TypeCloner`
//! is embedded, and its `replacement_for_null_scope`/`skip_lazy_type_clone`
//! fields carry the override state so the shared (non-virtual) `shallow_clone` /
//! `clone_children` machinery applies the fragment semantics to every node in the
//! cloned subgraph (free/table types take the fresh scope; lazy types are not
//! deep-cloned). `replacement_for_null_scope` is also mirrored on this record to
//! match the C++ member declaration surface.

use crate::records::{scope::Scope, type_cloner::TypeCloner};

#[derive(Debug)]
pub struct FragmentAutocompleteTypeCloner {
  pub(crate) base: TypeCloner,
  pub replacement_for_null_scope: *mut Scope,
}
