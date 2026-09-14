use crate::{
  records::{iterative_type_visitor::IterativeTypeVisitor, unknown_type::UnknownType},
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_id_unknown_type(&mut self, ty: TypeId, _utv: &UnknownType) -> bool {
    self.visit_type_id(ty)
  }
}
