use crate::{
  enums::table_state::TableState,
  records::{
    blocked_type::BlockedType, find_all_union_members::FindAllUnionMembers, free_type::FreeType,
    pending_expansion_type::PendingExpansionType, table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl FindAllUnionMembers {
  pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
    self.recorded_tys.insert_type_id(ty);
    false
  }

  pub fn visit_type_id_blocked_type(&mut self, _ty: TypeId, _btv: &BlockedType) -> bool {
    self.blocked_tys.insert_type_id(_ty);
    false
  }

  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    _petv: &PendingExpansionType,
  ) -> bool {
    self.blocked_tys.insert_type_id(_ty);
    false
  }

  pub fn visit_type_id_free_type(&mut self, _ty: TypeId, _ftv: &FreeType) -> bool {
    self.blocked_tys.insert_type_id(_ty);
    false
  }

  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    _ty: TypeId,
    _tfit: &TypeFunctionInstanceType,
  ) -> bool {
    self.blocked_tys.insert_type_id(_ty);
    false
  }

  pub fn visit_type_id_union_type(&mut self, _ty: TypeId, _ut: &UnionType) -> bool {
    true
  }

  pub fn visit_type_id_table_type(&mut self, _ty: TypeId, _tbl: &TableType) -> bool {
    if _tbl.state != TableState::Sealed {
      self.blocked_tys.insert_type_id(_ty);
    } else {
      self.recorded_tys.insert_type_id(_ty);
    }
    false
  }
}
