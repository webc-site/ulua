use crate::{
  records::{arena_id::ArenaId, clone_public_interface::ClonePublicInterface},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ClonePublicInterface {
  /// 跨模块（owning arena ≠ 本模块 internal_types arena）的类型不遍历子节点。
  /// `bool ClonePublicInterface::ignoreChildrenVisit(TypeId ty)`
  /// （`cpp/Analysis/src/Module.cpp:156`）。
  ///
  /// 降 safe 说明：`ty` 为 arena `TypeId` 句柄（同 `get_type_id` 门面纪律）；
  /// `self.module` 由 `Module::clone_public_interface` 以 `self as *mut Module`
  /// 接线，非空且存活至整个克隆遍历。两处解引用收进窄 `unsafe` 块。
  pub fn ignore_children_visit_type_id(&mut self, ty: TypeId) -> bool {
    // Safety: self.module 是模块克隆入口传入的存活 `*mut Module`，此处仅读
    // internal_types.arena_id 标识字段。
    let module = unsafe { &*self.module };

    let owning_arena = type_owning_arena(ty);
    owning_arena != module.internal_types.arena_id
  }

  /// 类型包孪生版。`bool ClonePublicInterface::ignoreChildrenVisit(TypePackId tp)`
  /// （`cpp/Analysis/src/Module.cpp:164`）。降 safe 理由同上。
  pub fn ignore_children_visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
    // Safety: 同 TypeId 版——self.module 为克隆入口传入的存活 `*mut Module`。
    let module = unsafe { &*self.module };
    let owning_arena = pack_owning_arena(tp);
    owning_arena != module.internal_types.arena_id
  }
}

/// TypeId 句柄 `owning_arena` 只读探针（cpp `ty->owningArena`，Module.cpp:157）：
/// 解引用收口在私有 helper、公共方法保持 safe，与
/// `clone_public_interface_is_dirty_module::type_owning_arena` 同一写法。
fn type_owning_arena(ty: TypeId) -> ArenaId {
  // SAFETY: ty 由克隆遍历保证为 arena 存活对齐节点（bump 块地址不移动），仅拷贝值。
  unsafe { (*ty).owning_arena }
}

/// TypePackId 孪生探针（Module.cpp:165）。
fn pack_owning_arena(tp: TypePackId) -> ArenaId {
  // SAFETY: tp 同上，为 pack arena 存活句柄，仅拷贝 owning_arena 值。
  unsafe { (*tp).owning_arena }
}
