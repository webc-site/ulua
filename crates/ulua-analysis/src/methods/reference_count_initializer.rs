//! `reference_count_initializer` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::table_state::TableState,
  records::{
    arena_handle::alias, blocked_type::BlockedType, blocked_type_pack::BlockedTypePack,
    extern_type::ExternType, free_type::FreeType, free_type_pack::FreeTypePack,
    pending_expansion_type::PendingExpansionType,
    reference_count_initializer::ReferenceCountInitializer, table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType, type_ids::TypeIds,
    type_once_visitor::TypeOnceVisitor,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId, type_pack_ids::TypePackIds},
};

impl ReferenceCountInitializer {
  pub fn reference_count_initializer_reference_count_initializer(
    mutated_types: *mut TypeIds,
    mutated_type_packs: *mut TypePackIds,
  ) -> Self {
    ReferenceCountInitializer {
      base: TypeOnceVisitor::new("ReferenceCountInitializer".to_string(), true),
      mutated_types,
      mutated_type_packs,
      traverse_into_type_functions: true,
    }
  }
}

impl ReferenceCountInitializer {
  /// C++ `bool ReferenceCountInitializer::visit(TypeId ty, const FreeType&)`
  /// (Constraint.cpp:26-30).
  pub fn visit_type_id_free_type(&mut self, ty: TypeId, _free_type: &FreeType) -> bool {
    alias(self.mutated_types).insert_type_id(ty);
    false
  }

  /// C++ `bool ReferenceCountInitializer::visit(TypeId ty, const BlockedType&)`
  /// (Constraint.cpp:32-36).
  pub fn visit_type_id_blocked_type(&mut self, ty: TypeId, _blocked_type: &BlockedType) -> bool {
    alias(self.mutated_types).insert_type_id(ty);
    false
  }

  /// C++ `bool ReferenceCountInitializer::visit(TypeId ty, const PendingExpansionType&)`
  /// (Constraint.cpp:38-42).
  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    _pending_expansion_type: &PendingExpansionType,
  ) -> bool {
    alias(self.mutated_types).insert_type_id(ty);
    false
  }

  pub fn visit_type_id_table_type(&mut self, ty: TypeId, tt: &TableType) -> bool {
    if matches!(tt.state, TableState::Unsealed | TableState::Free) {
      alias(self.mutated_types).order.push(ty);
    }

    true
  }

  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _extern_type: &ExternType) -> bool {
    false
  }

  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    _ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    tfit.function().can_reduce_generics
  }

  pub fn visit_type_pack_id_blocked_type_pack(
    &mut self,
    tp: TypePackId,
    _blocked_type_pack: &BlockedTypePack,
  ) -> bool {
    if fflag::LuauConstraintGraph.get() {
      LUAU_ASSERT!(!self.mutated_type_packs.is_null());
      alias(self.mutated_type_packs).insert(tp);
    }
    true
  }

  /// C++ `bool ReferenceCountInitializer::visit(TypePackId tp, const FreeTypePack&)`
  /// (Constraint.cpp:74-82).
  pub fn visit_type_pack_id_free_type_pack(
    &mut self,
    tp: TypePackId,
    _free_type_pack: &FreeTypePack,
  ) -> bool {
    if fflag::LuauConstraintGraph.get() {
      LUAU_ASSERT!(!self.mutated_type_packs.is_null());
      alias(self.mutated_type_packs).insert(tp);
    }
    true
  }
}
