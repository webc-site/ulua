use crate::{
  records::{function_type::FunctionType, iterative_type_visitor::IterativeTypeVisitor},
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_id_function_type(&mut self, ty: TypeId, _ftv: &FunctionType) -> bool {
    self.visit_type_id(ty)
  }
}
