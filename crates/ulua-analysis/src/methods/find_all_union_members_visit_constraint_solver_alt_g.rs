use crate::{
  enums::table_state::TableState,
  records::{find_all_union_members::FindAllUnionMembers, table_type::TableType},
  type_aliases::type_id::TypeId,
};

impl FindAllUnionMembers {
  pub fn visit_type_id_table_type(&mut self, _ty: TypeId, _tbl: &TableType) -> bool {
    if _tbl.state != TableState::Sealed {
      self.blocked_tys.insert_type_id(_ty);
    } else {
      self.recorded_tys.insert_type_id(_ty);
    }
    false
  }
}
