use crate::{
  records::{contains_refinable_type::ContainsRefinableType, negation_type::NegationType},
  type_aliases::type_id::TypeId,
};

impl ContainsRefinableType {
  pub fn visit_type_id_negation_type(&mut self, _ty: TypeId, _negation: &NegationType) -> bool {
    !self.found
  }
}
