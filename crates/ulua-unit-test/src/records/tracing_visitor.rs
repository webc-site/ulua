use alloc::{string::String, vec::Vec};

use ulua_analysis::{
  records::iterative_type_visitor::IterativeTypeVisitor, type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone)]
pub struct TracingVisitor {
  pub base: IterativeTypeVisitor,
  pub trace: Vec<String>,
  pub cycles: Vec<TypeId>,
}
