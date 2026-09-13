use alloc::{string::String, vec::Vec};

use ulua_analysis::records::iterative_type_visitor::IterativeTypeVisitor;

#[derive(Debug, Clone)]
pub struct TableSkippingVisitor {
  pub base: IterativeTypeVisitor,
  pub trace: Vec<String>,
}
