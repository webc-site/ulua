use crate::{
  records::{
    iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
    type_function_never_type::TypeFunctionNeverType,
  },
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

impl IterativeTypeFunctionTypeVisitor {
  pub fn visit_type_function_type_id_type_function_never_type(
    &mut self,
    ty: TypeFunctionTypeId,
    _tfnt: &TypeFunctionNeverType,
  ) -> bool {
    self.visit_type_function_type_id(ty)
  }
}
