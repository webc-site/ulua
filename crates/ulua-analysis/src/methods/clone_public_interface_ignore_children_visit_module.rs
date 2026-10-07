use crate::{
  records::{
    arena_handle::alias_ref, arena_id::ArenaId, clone_public_interface::ClonePublicInterface,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ClonePublicInterface {
  /// 跨模块（owning arena ≠ 本模块 internal_types arena）的类型不遍历子节点。
  /// `bool ClonePublicInterface::ignoreChildrenVisit(TypeId ty)`
  /// （`cpp/Analysis/src/Module.cpp:156`）。
  ///
  /// 降 safe 说明：`ty` 为 arena `TypeId` 句柄（同 `get_type_id` 门面纪律）；
  /// `self.module` 由 `Module::clone_public_interface` 以 `self as *mut Module`
  /// 接线，非空且存活至整个克隆遍历。
  pub fn ignore_children_visit_type_id(&mut self, ty: TypeId) -> bool {
    let module = alias_ref(self.module);

    let owning_arena = type_owning_arena(ty);
    owning_arena != module.internal_types.arena_id
  }

  /// 类型包孪生版。`bool ClonePublicInterface::ignoreChildrenVisit(TypePackId tp)`
  /// （`cpp/Analysis/src/Module.cpp:164`）。降 safe 理由同上。
  pub fn ignore_children_visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
    let module = alias_ref(self.module);
    let owning_arena = pack_owning_arena(tp);
    owning_arena != module.internal_types.arena_id
  }
}

/// TypeId 句柄 `owning_arena` 只读探针（cpp `ty->owningArena`，Module.cpp:157）：
/// 解引用收口在私有 helper、公共方法保持 safe，与
/// `clone_public_interface_is_dirty_module::type_owning_arena` 同一写法。
fn type_owning_arena(ty: TypeId) -> ArenaId {
  alias_ref(ty).owning_arena
}

/// TypePackId 孪生探针（Module.cpp:165）。
fn pack_owning_arena(tp: TypePackId) -> ArenaId {
  alias_ref(tp).owning_arena
}
