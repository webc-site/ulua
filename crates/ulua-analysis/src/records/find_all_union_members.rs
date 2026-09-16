use alloc::string::String;
use core::ffi::c_void;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::table_state::TableState,
  records::{
    blocked_type::BlockedType,
    free_type::FreeType,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    pending_expansion_type::PendingExpansionType,
    table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType,
    type_ids::TypeIds,
    type_once_visitor::TypeOnceVisitor,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone)]
pub struct FindAllUnionMembers {
  pub base: TypeOnceVisitor,
  pub recorded_tys: TypeIds,
  pub blocked_tys: TypeIds,
}

impl FindAllUnionMembers {
  pub fn new() -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("FindAllUnionMembers"), true),
      recorded_tys: TypeIds::new(),
      blocked_tys: TypeIds::new(),
    }
  }

  pub fn visit_blocked_type(&mut self, ty: TypeId, _btv: &BlockedType) -> bool {
    self.blocked_tys.insert_type_id(ty);
    false
  }

  pub fn visit_pending_expansion_type(&mut self, ty: TypeId, _petv: &PendingExpansionType) -> bool {
    self.blocked_tys.insert_type_id(ty);
    false
  }

  pub fn visit_free_type(&mut self, ty: TypeId, _ftv: &FreeType) -> bool {
    self.blocked_tys.insert_type_id(ty);
    false
  }

  pub fn visit_type_function_instance_type(
    &mut self,
    ty: TypeId,
    _tfit: &TypeFunctionInstanceType,
  ) -> bool {
    self.blocked_tys.insert_type_id(ty);
    false
  }

  pub fn visit_union_type(&mut self, _ty: TypeId, _ut: &UnionType) -> bool {
    true
  }

  pub fn visit_table_type(&mut self, ty: TypeId, tbl: &TableType) -> bool {
    if tbl.state != TableState::Sealed {
      self.blocked_tys.insert_type_id(ty);
    } else {
      self.recorded_tys.insert_type_id(ty);
    }
    false
  }
}

impl Default for FindAllUnionMembers {
  fn default() -> Self {
    Self::new()
  }
}

impl GenericTypeVisitorTrait for FindAllUnionMembers {
  type Seen = DenseHashSet<*mut c_void>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    self.recorded_tys.insert_type_id(ty);
    false
  }

  fn visit_type_id_blocked_type(&mut self, ty: TypeId, btv: &BlockedType) -> bool {
    self.visit_blocked_type(ty, btv)
  }

  fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    petv: &PendingExpansionType,
  ) -> bool {
    self.visit_pending_expansion_type(ty, petv)
  }

  fn visit_type_id_free_type(&mut self, ty: TypeId, ftv: &FreeType) -> bool {
    self.visit_free_type(ty, ftv)
  }

  fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    self.visit_type_function_instance_type(ty, tfit)
  }

  fn visit_type_id_union_type(&mut self, ty: TypeId, ut: &UnionType) -> bool {
    self.visit_union_type(ty, ut)
  }

  fn visit_type_id_table_type(&mut self, ty: TypeId, tbl: &TableType) -> bool {
    self.visit_table_type(ty, tbl)
  }
}
