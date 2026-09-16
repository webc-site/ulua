use alloc::{collections::BTreeMap, string::String};

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct NormalizedStringType {
  /// When false, this type represents a union of singleton string types.
  /// eg "a" | "b" | "c"
  ///
  /// When true, this type represents string intersected with negated string
  /// singleton types.
  /// eg string & ~"a" & ~"b" & ...
  pub(crate) is_cofinite: bool,

  pub(crate) singletons: BTreeMap<String, TypeId>,
}

impl NormalizedStringType {
  pub const NEVER: NormalizedStringType = NormalizedStringType {
    is_cofinite: false,
    singletons: BTreeMap::new(),
  };
}
