use crate::{
  records::{generic_type::GenericType, iterative_type_visitor::IterativeTypeVisitor},
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_id_generic_type(&mut self, ty: TypeId, _gtv: &GenericType) -> bool {
    self.visit_type_id(ty)
  }
}
