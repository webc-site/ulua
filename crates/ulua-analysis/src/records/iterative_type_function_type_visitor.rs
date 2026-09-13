use alloc::{string::String, vec::Vec};

use crate::{
  records::work_item_iterative_type_function_type_visitor::WorkItem,
  type_aliases::seen_set_iterative_type_function_type_visitor::SeenSet,
};

#[derive(Debug, Clone)]
pub struct IterativeTypeFunctionTypeVisitor {
  pub(crate) seen: SeenSet,
  pub(crate) work_queue: Vec<WorkItem>,
  pub(crate) parent_cursor: i32,
  pub(crate) work_cursor: u32,
  pub visitor_name: String,
  pub(crate) visit_once: bool,
}
