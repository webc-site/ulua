use crate::{
  records::{find_function_type_in::FindFunctionTypeIn, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

impl FindFunctionTypeIn {
  pub fn visit_type_id_union_type(&mut self, _ty: TypeId, _utv: &UnionType) -> bool {
    true
  }
}
