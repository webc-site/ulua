use crate::{
  records::{blocked_type::BlockedType, iterative_type_visitor::IterativeTypeVisitor},
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_id_blocked_type(&mut self, ty: TypeId, _btv: &BlockedType) -> bool {
    self.visit_type_id(ty)
  }
}
