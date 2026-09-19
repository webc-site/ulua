use crate::{
  records::{iterative_type_visitor::IterativeTypeVisitor, negation_type::NegationType},
  type_aliases::type_id::TypeId,
};

impl IterativeTypeVisitor {
  pub fn visit_type_id_negation_type(&mut self, ty: TypeId, _ntv: &NegationType) -> bool {
    self.visit_type_id(ty)
  }
}
