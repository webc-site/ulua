use alloc::string::String;

use crate::{
  enums::table_state::TableState,
  records::{
    extern_type::ExternType, function_type::FunctionType, generic_type::GenericType,
    generic_type_pack::GenericTypePack, table_type::TableType, type_once_visitor::TypeOnceVisitor,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct GenericTypeFinder {
  pub base: TypeOnceVisitor,
  pub found: bool,
}

impl GenericTypeFinder {
  pub fn generic_type_finder_generic_type_finder(&mut self) {
    self.found = false;
    self.base = TypeOnceVisitor::new(String::from("GenericTypeFinder"), true);
  }
}

impl GenericTypeFinder {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    !self.found
  }

  pub fn visit_type_pack_id(&mut self, _ty: TypePackId) -> bool {
    !self.found
  }

  pub fn visit_type_id_function_type(&mut self, _ty: TypeId, ftv: &FunctionType) -> bool {
    if ftv.has_no_free_or_generic_types {
      return false;
    }

    if !ftv.generics.is_empty() || !ftv.generic_packs.is_empty() {
      self.found = true;
    }

    !self.found
  }

  pub fn visit_type_id_table_type(&mut self, _ty: TypeId, ttv: &TableType) -> bool {
    // C++ `bool visit(TypeId, const Luau::TableType& ttv)` (Instantiation.h:122-128):
    // a generic table forces instantiation, so mark it found.
    if ttv.state == TableState::Generic {
      self.found = true;
    }

    !self.found
  }

  pub fn visit_type_id_generic_type(&mut self, _ty: TypeId, _gtv: &GenericType) -> bool {
    self.found = true;
    false
  }

  pub fn visit_type_pack_id_generic_type_pack(
    &mut self,
    _ty: TypePackId,
    _gtp: &GenericTypePack,
  ) -> bool {
    self.found = true;
    false
  }

  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
    // During function instantiation, extern types are not traversed even if they have generics
    false
  }
}
