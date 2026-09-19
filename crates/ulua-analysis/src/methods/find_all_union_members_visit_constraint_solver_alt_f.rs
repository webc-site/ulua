use crate::{
  records::{find_all_union_members::FindAllUnionMembers, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

impl FindAllUnionMembers {
  pub fn visit_type_id_union_type(&mut self, _ty: TypeId, _ut: &UnionType) -> bool {
    true
  }
}
