//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/VisitType.h:83:generic_type_visitor_generic_type_visitor`
//! Source: `Analysis/include/Luau/VisitType.h:83-88` (hand-ported)

use alloc::string::String;

use crate::records::generic_type_visitor::GenericTypeVisitor;

impl<S> GenericTypeVisitor<S> {
  /// C++ `explicit GenericTypeVisitor(const std::string visitorName, Set seen, bool skipBoundTypes = false)`.
  pub fn generic_type_visitor_string_set_bool(
    visitor_name: String,
    seen: S,
    skip_bound_types: bool,
  ) -> Self {
    Self {
      visitor_name,
      seen,
      skip_bound_types,
      recursion_counter: 0,
      type_function_depth: 0,
    }
  }
}
