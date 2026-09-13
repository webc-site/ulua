use crate::{
  records::iterative_type_visitor::IterativeTypeVisitor,
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};

impl IterativeTypeVisitor {
  pub fn visit_type_id_error_type(&mut self, ty: TypeId, _etv: &ErrorType) -> bool {
    self.visit_type_id(ty)
  }
}
