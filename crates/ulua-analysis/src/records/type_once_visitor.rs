//! Source: `Analysis/include/Luau/VisitType.h:522-529` (hand-ported)
//!
//! C++ `struct TypeOnceVisitor : GenericTypeVisitor<DenseHashSet<void*>>`.
//! Each type is checked once even if there are multiple paths to it (the
//! DenseHashSet seen set never forgets — `unsee` is a no-op).

use alloc::string::String;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::generic_type_visitor::GenericTypeVisitor;
#[derive(Debug, Clone)]
pub struct TypeOnceVisitor {
  pub base: GenericTypeVisitor<DenseHashSet<*mut ()>>,
}

impl TypeOnceVisitor {
  /// C++ `explicit TypeOnceVisitor(const std::string visitorName, bool skipBoundTypes)`
  /// — seeds the seen set with the `nullptr` empty key, as in
  /// `DenseHashSet<void*>{nullptr}`.
  pub fn new(visitor_name: String, skip_bound_types: bool) -> Self {
    Self {
      base: GenericTypeVisitor::generic_type_visitor_string_set_bool(
        visitor_name,
        DenseHashSet::default(),
        skip_bound_types,
      ),
    }
  }
}
