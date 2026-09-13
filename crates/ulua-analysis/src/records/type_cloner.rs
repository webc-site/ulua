//! Source: `Analysis/src/Clone.cpp:25` (hand-ported; fields only)
//! Clone.h:15: using SeenTypes = std::unordered_map<TypeId, TypeId>;
//! Clone.h:16: using SeenTypePacks = std::unordered_map<TypePackId, TypePackId>;

use alloc::vec::Vec;
use std::collections::HashMap;

use crate::{
  records::{builtin_types::BuiltinTypes, scope::Scope, type_arena::TypeArena},
  type_aliases::{type_id::TypeId, type_or_pack::TypeOrPack, type_pack_id::TypePackId},
};

pub type SeenTypes = HashMap<TypeId, TypeId>;
pub type SeenTypePacks = HashMap<TypePackId, TypePackId>;

#[derive(Debug)]
pub struct TypeCloner {
  pub arena: *mut TypeArena,
  pub builtin_types: *mut BuiltinTypes,
  pub queue: Vec<TypeOrPack>,
  pub types: *mut SeenTypes,
  pub packs: *mut SeenTypePacks,
  pub force_ty: TypeId,
  pub force_tp: TypePackId,
  pub steps: i32,
  /// Subclass state carried by `FragmentAutocompleteTypeCloner` (Clone.cpp:473).
  /// In C++ the fragment cloner is a `TypeCloner` subclass whose `shallowClone`
  /// override (Clone.cpp:493-518) substitutes this scope for a null free/table
  /// scope, and whose `cloneChildren(LazyType*)` override (Clone.cpp:541-544)
  /// is a no-op. Because the recursive `cloneChildren` machinery dispatches
  /// through the virtual `shallowClone`, that substitution must apply to every
  /// node cloned in the subgraph, not just the root. The Rust port has no
  /// vtable, so the divergence is encoded as cloner state read by the (shared)
  /// `shallowClone`/`cloneChildren` methods. Both fields are inert for the
  /// non-fragment callers (null scope == prior behaviour, skip flag == false).
  pub replacement_for_null_scope: *mut Scope,
  pub skip_lazy_type_clone: bool,
}
