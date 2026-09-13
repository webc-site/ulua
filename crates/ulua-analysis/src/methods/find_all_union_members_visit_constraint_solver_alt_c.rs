use crate::{
  records::{
    find_all_union_members::FindAllUnionMembers, pending_expansion_type::PendingExpansionType,
  },
  type_aliases::type_id::TypeId,
};

impl FindAllUnionMembers {
  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    _petv: &PendingExpansionType,
  ) -> bool {
    self.blocked_tys.insert_type_id(_ty);
    false
  }
}
