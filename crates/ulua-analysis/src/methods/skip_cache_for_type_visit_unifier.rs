use crate::{
  enums::table_state::TableState,
  functions::get_mutable_type,
  records::{
    arena_id::ArenaId, blocked_type::BlockedType, blocked_type_pack::BlockedTypePack,
    free_type::FreeType, free_type_pack::FreeTypePack, generic_type::GenericType,
    generic_type_pack::GenericTypePack, pending_expansion_type::PendingExpansionType,
    skip_cache_for_type::SkipCacheForType, table_type::TableType,
  },
  type_aliases::{
    bound_type::BoundType, bound_type_pack::BoundTypePack, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

impl SkipCacheForType {
  pub fn visit_type_id_free_type(&mut self, _ty: TypeId, _ft: &FreeType) -> bool {
    self.result = true;
    false
  }

  pub fn visit_type_id_bound_type(&mut self, _ty: TypeId, _bt: &BoundType) -> bool {
    self.result = true;
    false
  }

  pub fn visit_type_id_generic_type(&mut self, _ty: TypeId, _gt: &GenericType) -> bool {
    self.result = true;
    false
  }

  pub fn visit_type_id_blocked_type(&mut self, _ty: TypeId, _bt: &BlockedType) -> bool {
    self.result = true;
    false
  }

  pub fn visit_type_id_pending_expansion_type(
    &mut self,
    _ty: TypeId,
    _pet: &PendingExpansionType,
  ) -> bool {
    self.result = true;
    false
  }

  /// 对应 cpp `SkipCacheForType::visit(TypeId, const TableType&)`
  /// （`Analysis/src/Unifier.cpp:181`）。降 safe：
  /// `ty` 为 traverse 遍历传入的 arena `TypeId` 句柄（同 `get_mutable_type_id`
  /// 门面纪律），解引用收进窄 `unsafe` 块，_tt 引用形态与其余 `visit_type_id_*`
  /// 覆写一致（形状钉死于 TypeVarVisitor 分派签名，但本函数自身无调用方契约）。
  pub fn visit_type_id_table_type(&mut self, ty: TypeId, _tt: &TableType) -> bool {
    if type_owning_arena(ty) != self.type_arena_id {
      return false;
    }
    if let Some(ttv) = get_mutable_type::get_mutable::<TableType>(ty)
      && (ttv.bound_to.is_some() || ttv.state != TableState::Sealed)
    {
      self.result = true;
      return false;
    }
    true
  }

  /// 降 safe 说明（原 `# Safety` 占位删除）：`ty` 为 arena `TypeId` 句柄，
  /// `self.skip_cache_for_type` 由构造入参接线为
  /// `&shared_state.skip_cache_for_type`（UnifierSharedState 内嵌 DenseHashMap，
  /// 比遍历长寿）；裸引用均已在下方窄 `unsafe` 块内证成。
  pub(crate) fn visit_type_id(&mut self, ty: TypeId) -> bool {
    // Safety: ty 是遍历器传入的有效 *mut Type（type_arena 存活对齐节点，地址不移动）。
    // self.skip_cache_for_type 由构造入参接线
    // 为 &shared_state.skip_cache_for_type，指向 UnifierSharedState 内嵌的 DenseHashMap，
    // 在本次遍历借用期内非空且存活，故 (*self.skip_cache_for_type).find 只读查找合法。
    let in_arena = type_owning_arena(ty) == self.type_arena_id;
    // Safety: self.skip_cache_for_type 由构造入参接线为 &shared_state.skip_cache_for_type
    // （UnifierSharedState 内嵌 DenseHashMap，比遍历长寿），只读查找。
    unsafe {
      if !in_arena {
        return false;
      }
      if let Some(prev) = (*self.skip_cache_for_type).find(&ty)
        && *prev
      {
        self.result = true;
        return false;
      }
    }
    true
  }

  /// 对应 cpp `SkipCacheForType::visit(TypePackId tp)`（`Analysis/src/Unifier.cpp:221`）。
  /// 降 safe：`tp` 为 arena `TypePackId` 句柄（同 `get_type_pack_id` 门面纪律），
  /// owning_arena 比对收进窄 `unsafe` 块，调用方无需契约。
  pub fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
    pack_owning_arena(tp) == self.type_arena_id
  }

  pub fn visit_type_pack_id_free_type_pack(
    &mut self,
    _tp: TypePackId,
    _ftp: &FreeTypePack,
  ) -> bool {
    self.result = true;
    false
  }

  pub fn visit_type_pack_id_bound_type_pack(
    &mut self,
    _tp: TypePackId,
    _btp: &BoundTypePack,
  ) -> bool {
    self.result = true;
    false
  }

  pub fn visit_type_pack_id_generic_type_pack(
    &mut self,
    _tp: TypePackId,
    _gtp: &GenericTypePack,
  ) -> bool {
    self.result = true;
    false
  }

  pub fn visit_type_pack_id_blocked_type_pack(
    &mut self,
    _tp: TypePackId,
    _btp: &BlockedTypePack,
  ) -> bool {
    self.result = true;
    false
  }
}

/// arena 句柄 `owning_arena` 只读探针（解引用收口进私有 helper，公共方法保持 safe，
/// 同 `clone_clone::type_is_persistent` 写法）；句柄有效性沿用本 crate TypeId 纪律。
fn type_owning_arena(ty: TypeId) -> ArenaId {
  // SAFETY: ty 为遍历器传入的 arena 存活对齐节点（bump 块地址不移动），仅拷贝值。
  unsafe { (*ty).owning_arena }
}

fn pack_owning_arena(tp: TypePackId) -> ArenaId {
  // SAFETY: tp 同上，为 pack arena 存活句柄，仅拷贝 owning_arena。
  unsafe { (*tp).owning_arena }
}
