use crate::{
  records::{negation_type::NegationType, negation_type_finder::NegationTypeFinder},
  type_aliases::type_id::TypeId,
};

impl NegationTypeFinder {
  pub fn visit_type_id_negation_type(&mut self, _ty: TypeId, _ntv: &NegationType) -> bool {
    self.found = true;
    !self.found
  }
}
