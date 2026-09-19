use crate::{records::find_all_union_members::FindAllUnionMembers, type_aliases::type_id::TypeId};

impl FindAllUnionMembers {
  pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
    self.recorded_tys.insert_type_id(ty);
    false
  }
}
