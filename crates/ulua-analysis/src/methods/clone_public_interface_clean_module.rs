use core::ptr::null_mut;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::table_state::TableState,
  functions::{get_mutable_type, get_mutable_type_pack, get_type, get_type_pack},
  records::{
    blocked_type::BlockedType,
    blocked_type_pack::BlockedTypePack,
    clone_public_interface::ClonePublicInterface,
    extern_type::ExternType,
    free_type::FreeType,
    free_type_pack::FreeTypePack,
    function_type::FunctionType,
    generic_type::GenericType,
    generic_type_finder::GenericTypeFinder,
    generic_type_pack::GenericTypePack,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    pending_expansion_type::PendingExpansionType,
    table_type::TableType,
    type_level::TypeLevel,
    visit_key::VisitKey,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl GenericTypeVisitorTrait for GenericTypeFinder {
  type Seen = DenseHashSet<VisitKey>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    GenericTypeFinder::visit_type_id(self, ty)
  }

  fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
    GenericTypeFinder::visit_type_pack_id(self, tp)
  }

  fn visit_type_id_function_type(&mut self, ty: TypeId, ftv: &FunctionType) -> bool {
    GenericTypeFinder::visit_type_id_function_type(self, ty, ftv)
  }

  fn visit_type_id_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
    GenericTypeFinder::visit_type_id_table_type(self, ty, ttv)
  }

  fn visit_type_id_generic_type(&mut self, ty: TypeId, gtv: &GenericType) -> bool {
    GenericTypeFinder::visit_type_id_generic_type(self, ty, gtv)
  }

  fn visit_type_pack_id_generic_type_pack(
    &mut self,
    tp: TypePackId,
    gtp: &GenericTypePack,
  ) -> bool {
    GenericTypeFinder::visit_type_pack_id_generic_type_pack(self, tp, gtp)
  }

  fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
    GenericTypeFinder::visit_type_id_extern_type(self, ty, etv)
  }
}

impl ClonePublicInterface {
  /// `TypeId ClonePublicInterface::clean(TypeId ty)`.
  /// Reference: `Module.cpp:167-208`.
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    let mut result = self.base.clone_type_id(ty);

    if let Some(ftv) = get_mutable_type::get_mutable::<FunctionType>(result) {
      if ftv.generics.is_empty() && ftv.generic_packs.is_empty() {
        let mut marker = GenericTypeFinder::new();
        marker.traverse_type_id(result);

        if !marker.found {
          ftv.has_no_free_or_generic_types = true;
        }
      }

      ftv.level = TypeLevel::new(0, 0);
    } else if let Some(ttv) = get_mutable_type::get_mutable::<TableType>(result) {
      ttv.level = TypeLevel::new(0, 0);
      if self.is_new_solver() {
        // 导出接口前清回落回全局作用域：`scope: *mut Scope` 为 arena 记录
        // 既有字段约定（空即「无量化作用域」，见 generic_type ctor 注释头），
        // 此处仅写哨兵，不改字段类型。
        ttv.scope = null_mut();
        ttv.state = TableState::Sealed;
      }
    }

    if self.is_new_solver() {
      if get_type::get::<FreeType>(ty).is_some()
        || get_type::get::<BlockedType>(ty).is_some()
        || get_type::get::<PendingExpansionType>(ty).is_some()
      {
        self.internal_type_escaped = true;
        // SAFETY: builtin_types 指向会话期 BuiltinTypes，全局内建类型表。
        result = self.builtin_types.get_mut().error_type;
      } else if let Some(genericty) = get_mutable_type::get_mutable::<GenericType>(result) {
        // 已提升为全局的泛型清回无作用域态（同上 `scope: *mut Scope` 字段约定）。
        genericty.scope = null_mut();
      }
    }

    result
  }

  /// `TypePackId ClonePublicInterface::clean(TypePackId tp)`.
  /// Reference: `Module.cpp:210-229`.
  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    if self.is_new_solver() {
      if get_type_pack::get::<FreeTypePack>(tp).is_some()
        || get_type_pack::get::<BlockedTypePack>(tp).is_some()
      {
        self.internal_type_escaped = true;
        // SAFETY: builtin_types 指向全局 BuiltinTypes，存活期覆盖整个分析过程。
        return self.builtin_types.get_mut().error_type_pack;
      }

      let cloned_tp = self.base.clone_type_pack_id(tp);
      if let Some(gtp) = get_mutable_type_pack::get_mutable::<GenericTypePack>(cloned_tp) {
        // 同 type 侧：清回「无量化作用域」哨兵，属 `scope: *mut Scope` 字段约定。
        gtp.scope = null_mut();
      }
      cloned_tp
    } else {
      self.base.clone_type_pack_id(tp)
    }
  }
}
