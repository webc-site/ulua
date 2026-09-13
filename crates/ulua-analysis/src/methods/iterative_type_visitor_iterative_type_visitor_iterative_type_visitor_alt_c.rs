use crate::records::iterative_type_visitor::IterativeTypeVisitor;

impl IterativeTypeVisitor {
  pub fn iterative_type_visitor_string_bool(&mut self, visitor_name: &str, skip_bound_types: bool) {
    self.iterative_type_visitor_string_bool_bool(visitor_name, true, skip_bound_types);
  }
}
