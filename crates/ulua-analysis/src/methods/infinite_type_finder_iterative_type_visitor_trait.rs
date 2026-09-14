use crate::{
  records::{
    infinite_type_finder::InfiniteTypeFinder,
    iterative_type_visitor::{IterativeTypeVisitor, IterativeTypeVisitorTrait},
    pending_expansion_type::PendingExpansionType,
  },
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitorTrait for InfiniteTypeFinder {
  fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
    &mut self.base
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    InfiniteTypeFinder::visit_type_id(self, ty)
  }

  fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    petv: &PendingExpansionType,
  ) -> bool {
    InfiniteTypeFinder::visit_type_id_pending_expansion_type(self, ty, petv)
  }
}
