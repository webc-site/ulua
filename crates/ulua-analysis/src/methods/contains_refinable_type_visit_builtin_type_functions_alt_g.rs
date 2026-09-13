use crate::{
  records::{contains_refinable_type::ContainsRefinableType, intersection_type::IntersectionType},
  type_aliases::type_id::TypeId,
};

impl ContainsRefinableType {
  pub fn visit_type_id_intersection_type(
    &mut self,
    _ty: TypeId,
    _intersection: &IntersectionType,
  ) -> bool {
    !self.found
  }
}
