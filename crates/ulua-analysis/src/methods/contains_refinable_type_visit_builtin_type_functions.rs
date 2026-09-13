use crate::{
  records::contains_refinable_type::ContainsRefinableType, type_aliases::type_id::TypeId,
};

impl ContainsRefinableType {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    // Default case: if we find *some* type that's worth refining against,
    // then we can claim that this type contains a refineable type.
    self.found = true;
    false
  }
}
