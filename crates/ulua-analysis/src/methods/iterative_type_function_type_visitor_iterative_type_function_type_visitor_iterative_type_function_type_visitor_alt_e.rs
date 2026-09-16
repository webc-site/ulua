use alloc::{string::String, vec::Vec};

use crate::{
  records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
  type_aliases::seen_set_iterative_type_function_type_visitor::SeenSet,
};

impl IterativeTypeFunctionTypeVisitor {
  pub fn iterative_type_function_type_visitor_string_seen_set_bool(
    visitor_name: String,
    seen: SeenSet,
    visit_once: bool,
  ) -> Self {
    // Skip the first few doublings.  Almost all visits require less than 32 steps.
    let work_queue = Vec::with_capacity(32);

    IterativeTypeFunctionTypeVisitor {
      seen,
      work_queue,
      parent_cursor: -1,
      work_cursor: 0,
      visitor_name,
      visit_once,
    }
  }
}
