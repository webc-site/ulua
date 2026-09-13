use crate::{
  records::{iterative_type_visitor::IterativeTypeVisitor, variadic_type_pack::VariadicTypePack},
  type_aliases::type_pack_id::TypePackId,
};
impl IterativeTypeVisitor {
  pub fn visit_type_pack_id_variadic_type_pack(
    &mut self,
    tp: TypePackId,
    _vtp: &VariadicTypePack,
  ) -> bool {
    self.visit_type_pack_id(tp)
  }
}
