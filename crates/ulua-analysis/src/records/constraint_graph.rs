use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    constraint_list::ConstraintList,
    hash_blocked_constraint_id::HashBlockedConstraintId,
    slot_arena::{SlotArena, SlotId},
  },
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

/// C++ `ConstraintGraph::dependencies` 的映射类型（Analysis/include/Luau/ConstraintGraph.h）。
///
/// §2 句柄化：值由 `ConstraintList*`（`PinnedStorage` 的 Box 堆址）改为
/// [`SlotArena`] 发放的 `SlotId` 索引，节点身份不再依赖堆地址。
pub type ConstraintMap = DenseHashMap<BlockedConstraintId, SlotId, HashBlockedConstraintId>;

#[derive(Debug, Clone)]
pub struct ConstraintGraph {
  pub(crate) dependencies: ConstraintMap,
  pub(crate) reverse_dependencies: ConstraintMap,
  /// 依赖列表存储：只追加、槽号永不复用，故句柄跨 `&mut` 借用依然有效。
  pub(crate) constraint_lists: SlotArena<ConstraintList>,
}
