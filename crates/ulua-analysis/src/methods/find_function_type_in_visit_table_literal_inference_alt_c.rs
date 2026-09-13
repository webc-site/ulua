use crate::{
  records::{find_function_type_in::FindFunctionTypeIn, intersection_type::IntersectionType},
  type_aliases::type_id::TypeId,
};

impl FindFunctionTypeIn {
  pub fn visit_type_id_intersection_type(&mut self, _ty: TypeId, _itv: &IntersectionType) -> bool {
    true
  }
}
