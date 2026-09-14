use crate::{
  records::iterative_type_visitor::IterativeTypeVisitor,
  type_aliases::{error_type_pack::ErrorTypePack, type_pack_id::TypePackId},
};

impl IterativeTypeVisitor {
  pub fn visit_type_pack_id_error_type_pack(
    &mut self,
    tp: TypePackId,
    _etp: &ErrorTypePack,
  ) -> bool {
    self.visit_type_pack_id(tp)
  }
}
