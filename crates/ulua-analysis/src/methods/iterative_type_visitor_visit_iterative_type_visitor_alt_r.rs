use crate::{
  records::{
    iterative_type_visitor::IterativeTypeVisitor, pending_expansion_type::PendingExpansionType,
  },
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    _petv: &PendingExpansionType,
  ) -> bool {
    self.visit_type_id(ty)
  }
}
