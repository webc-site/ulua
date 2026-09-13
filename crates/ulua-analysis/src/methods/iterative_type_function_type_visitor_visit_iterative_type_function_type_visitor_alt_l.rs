use crate::{
  records::{
    iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor,
    type_function_extern_type::TypeFunctionExternType,
  },
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

impl IterativeTypeFunctionTypeVisitor {
  pub fn visit_type_function_type_id_type_function_extern_type(
    &mut self,
    ty: TypeFunctionTypeId,
    tfet: &TypeFunctionExternType,
  ) -> bool {
    let _ = tfet;
    self.visit_type_function_type_id(ty)
  }
}
