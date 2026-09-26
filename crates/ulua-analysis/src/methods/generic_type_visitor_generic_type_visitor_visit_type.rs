use alloc::string::String;

use crate::records::generic_type_visitor::GenericTypeVisitor;

impl<S: Default> GenericTypeVisitor<S> {
  /// C++ `GenericTypeVisitor() = default;`
  pub fn new() -> Self {
    Self {
      visitor_name: String::new(),
      seen: S::default(),
      skip_bound_types: false,
      recursion_counter: 0,
      type_function_depth: 0,
    }
  }
}

impl<S: Default> Default for GenericTypeVisitor<S> {
  fn default() -> Self {
    Self::new()
  }
}

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
