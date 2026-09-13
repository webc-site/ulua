use crate::{
  records::{blocked_type::BlockedType, find_all_union_members::FindAllUnionMembers},
  type_aliases::type_id::TypeId,
};

impl FindAllUnionMembers {
  pub fn visit_type_id_blocked_type(&mut self, _ty: TypeId, _btv: &BlockedType) -> bool {
    self.blocked_tys.insert_type_id(_ty);
    false
  }
}
