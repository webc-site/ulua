use crate::{
  records::{iterative_type_visitor::IterativeTypeVisitor, singleton_type::SingletonType},
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_id_singleton_type(&mut self, ty: TypeId, _stv: &SingletonType) -> bool {
    self.visit_type_id(ty)
  }
}
