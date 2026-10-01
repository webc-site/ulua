//! `find_simplification_blockers` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::{
  records::{
    blocked_type::BlockedType,
    extern_type::ExternType,
    find_simplification_blockers::FindSimplificationBlockers,
    free_type::FreeType,
    function_type::FunctionType,
    iterative_type_visitor::{IterativeTypeVisitor, IterativeTypeVisitorTrait},
    pending_expansion_type::PendingExpansionType,
  },
  type_aliases::type_id::TypeId,
};

impl FindSimplificationBlockers {
  pub fn find_simplification_blockers_find_simplification_blockers(&mut self) {
    self
      .base
      .iterative_type_visitor_string_bool("FindSimplificationBlockers", true);
  }
}

impl IterativeTypeVisitorTrait for FindSimplificationBlockers {
  fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
    &mut self.base
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    FindSimplificationBlockers::visit_type_id(self, ty)
  }

  fn visit_type_id_blocked_type(&mut self, ty: TypeId, btv: &BlockedType) -> bool {
    FindSimplificationBlockers::visit_type_id_blocked_type(self, ty, btv)
  }

  fn visit_type_id_free_type(&mut self, ty: TypeId, ftv: &FreeType) -> bool {
    FindSimplificationBlockers::visit_type_id_free_type(self, ty, ftv)
  }

  fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    petv: &PendingExpansionType,
  ) -> bool {
    FindSimplificationBlockers::visit_type_id_pending_expansion_type(self, ty, petv)
  }

  fn visit_type_id_function_type(&mut self, ty: TypeId, ftv: &FunctionType) -> bool {
    FindSimplificationBlockers::visit_type_id_function_type(self, ty, ftv)
  }

  fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
    FindSimplificationBlockers::visit_type_id_extern_type(self, ty, etv)
  }
}

impl FindSimplificationBlockers {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    !self.found
  }

  pub fn visit_type_id_blocked_type(&mut self, _ty: TypeId, _btv: &BlockedType) -> bool {
    self.found = true;
    false
  }

  pub fn visit_type_id_free_type(&mut self, _ty: TypeId, _ftv: &FreeType) -> bool {
    self.found = true;
    false
  }

  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    _petv: &PendingExpansionType,
  ) -> bool {
    self.found = true;
    false
  }

  pub fn visit_type_id_function_type(&mut self, _ty: TypeId, _ftv: &FunctionType) -> bool {
    false
  }

  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
    false
  }
}
