use crate::{
  records::{
    contains_refinable_type::ContainsRefinableType, function_type::FunctionType,
    intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, no_refine_type::NoRefineType, table_type::TableType,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl ContainsRefinableType {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    // Default case: if we find *some* type that's worth refining against,
    // then we can claim that this type contains a refineable type.
    self.found = true;
    false
  }

  pub fn visit_type_id_no_refine_type(&mut self, _ty: TypeId, _nrt: &NoRefineType) -> bool {
    false
  }

  pub fn visit_type_id_table_type(&mut self, _ty: TypeId, _table: &TableType) -> bool {
    !self.found
  }

  pub fn visit_type_id_metatable_type(&mut self, _ty: TypeId, _metatable: &MetatableType) -> bool {
    !self.found
  }

  pub fn visit_type_id_function_type(
    &mut self,
    _ty: TypeId,
    _function_type: &FunctionType,
  ) -> bool {
    !self.found
  }

  pub fn visit_type_id_union_type(&mut self, _ty: TypeId, _union: &UnionType) -> bool {
    !self.found
  }

  pub fn visit_type_id_intersection_type(
    &mut self,
    _ty: TypeId,
    _intersection: &IntersectionType,
  ) -> bool {
    !self.found
  }

  pub fn visit_type_id_negation_type(&mut self, _ty: TypeId, _negation: &NegationType) -> bool {
    !self.found
  }
}
