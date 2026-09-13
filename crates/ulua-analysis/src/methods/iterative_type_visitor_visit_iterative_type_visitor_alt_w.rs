use crate::{
  records::iterative_type_visitor::IterativeTypeVisitor,
  type_aliases::{bound_type_pack::BoundTypePack, type_pack_id::TypePackId},
};

impl IterativeTypeVisitor {
  pub fn visit_type_pack_id_bound_type_pack(
    &mut self,
    tp: TypePackId,
    _btp: &BoundTypePack,
  ) -> bool {
    self.visit_type_pack_id(tp)
  }
}
