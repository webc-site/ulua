use crate::{
  records::{blocked_type_pack::BlockedTypePack, iterative_type_visitor::IterativeTypeVisitor},
  type_aliases::type_pack_id::TypePackId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_pack_id_blocked_type_pack(
    &mut self,
    tp: TypePackId,
    _btp: &BlockedTypePack,
  ) -> bool {
    self.visit_type_pack_id(tp)
  }
}
