use crate::{
  records::{any_type::AnyType, iterative_type_visitor::IterativeTypeVisitor},
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_id_any_type(&mut self, ty: TypeId, _atv: &AnyType) -> bool {
    self.visit_type_id(ty)
  }
}
