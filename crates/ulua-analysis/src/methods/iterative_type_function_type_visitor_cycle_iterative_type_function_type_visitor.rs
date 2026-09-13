use crate::{
  records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

impl IterativeTypeFunctionTypeVisitor {
  pub fn cycle_type_function_type_id(&mut self, _ty: TypeFunctionTypeId) {
    // Empty implementation per source: void IterativeTypeFunctionTypeVisitor::cycle(TypeFunctionTypeId) {}
  }
}
