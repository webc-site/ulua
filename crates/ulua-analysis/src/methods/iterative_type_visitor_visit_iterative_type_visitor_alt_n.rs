use crate::{
  records::{iterative_type_visitor::IterativeTypeVisitor, never_type::NeverType},
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_id_never_type(&mut self, ty: TypeId, _ntv: &NeverType) -> bool {
    self.visit_type_id(ty)
  }
}
