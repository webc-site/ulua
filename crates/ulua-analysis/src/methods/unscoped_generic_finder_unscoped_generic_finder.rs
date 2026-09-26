use crate::records::{
  type_once_visitor::TypeOnceVisitor, unscoped_generic_finder::UnscopedGenericFinder,
};

impl UnscopedGenericFinder {
  pub fn new() -> Self {
    UnscopedGenericFinder {
      base: TypeOnceVisitor::new("UnscopedGenericFinder".to_string(), true),
      scope_gen_tys: Vec::new(),
      scope_gen_tps: Vec::new(),
      found_unscoped: false,
    }
  }
}

impl Default for UnscopedGenericFinder {
  fn default() -> Self {
    Self::new()
  }
}
