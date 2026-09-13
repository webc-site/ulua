use crate::{
  records::{
    iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
    type_function_singleton_type::TypeFunctionSingletonType,
  },
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

impl IterativeTypeFunctionTypeVisitor {
  pub fn visit_type_function_type_id_type_function_singleton_type(
    &mut self,
    ty: TypeFunctionTypeId,
    _tfst: &TypeFunctionSingletonType,
  ) -> bool {
    self.visit_type_function_type_id(ty)
  }
}
