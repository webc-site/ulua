use alloc::vec::Vec;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TablePropLookupResult {
  /// What types are we blocked on for determining this type?
  pub blocked_types: Vec<TypeId>,
  /// The type of the property (if we were able to determine it).
  pub prop_type: Option<TypeId>,
  /// Whether or not this is _definitely_ derived as the result of an indexer.
  /// We use this to determine whether or not code like:
  ///
  ///   t.lol = nil;
  ///
  /// ... is legal. If `t: { [string]: ~nil }` then this is legal as
  /// there's no guarantee on whether "lol" specifically exists.
  /// However, if `t: { lol: ~nil }`, then we cannot allow assignment as
  /// that would remove "lol" from the table entirely.
  pub is_index: bool,
}

unsafe impl Send for TablePropLookupResult {}
unsafe impl Sync for TablePropLookupResult {}
