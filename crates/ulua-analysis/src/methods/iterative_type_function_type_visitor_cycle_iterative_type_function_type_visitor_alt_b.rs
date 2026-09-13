use crate::{
  records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
  type_aliases::type_function_type_pack_id::TypeFunctionTypePackId,
};

impl IterativeTypeFunctionTypeVisitor {
  pub fn cycle_type_function_type_pack_id(&mut self, _tp: TypeFunctionTypePackId) {
    // Empty implementation per source: void IterativeTypeFunctionTypeVisitor::cycle(TypeFunctionTypePackId) {}
  }
}
