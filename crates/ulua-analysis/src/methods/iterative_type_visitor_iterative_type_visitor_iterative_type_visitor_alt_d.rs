use core::ptr::null_mut;

use crate::{
  records::iterative_type_visitor::IterativeTypeVisitor,
  type_aliases::seen_set_iterative_type_visitor::SeenSet,
};
impl IterativeTypeVisitor {
  pub fn iterative_type_visitor_string_bool_bool(
    &mut self,
    visitor_name: &str,
    visit_once: bool,
    skip_bound_types: bool,
  ) {
    self.iterative_type_visitor_string_seen_set_bool_bool(
      visitor_name,
      SeenSet::new(null_mut()),
      visit_once,
      skip_bound_types,
    );
  }
}
