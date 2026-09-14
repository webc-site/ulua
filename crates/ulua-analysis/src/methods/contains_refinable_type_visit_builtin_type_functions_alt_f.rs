use crate::{
  records::{contains_refinable_type::ContainsRefinableType, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

impl ContainsRefinableType {
  pub fn visit_type_id_union_type(&mut self, _ty: TypeId, _union: &UnionType) -> bool {
    !self.found
  }
}
