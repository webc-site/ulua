use crate::{
  functions::{follow_type, follow_type_pack},
  records::{blocked_constraint_registry::ConstraintId, constraint_graph::ConstraintGraph},
  type_aliases::{
    blocked_constraint_id::BlockedConstraintId, type_id::TypeId, type_pack_id::TypePackId,
  },
};

impl ConstraintGraph {
  pub fn unblock_type_or_pack_type_id(&mut self, vertex: TypeId) {
    self.repair_type_references_type_id(vertex);
    let followed = follow_type::follow(vertex);
    self.clear_reverse_dependencies_of(BlockedConstraintId::V0(followed));
  }

  /// # Safety
  /// `vertex` 须为指向存活类型/类型包 arena 节点的有效 `TypePackId`；解阻塞过程会经 `self` 内部
  /// 的 `log`/依赖表裸指针（构造时接线自存活 `TxnLog`/builder，比本图长寿）读取并就地更新。
  /// 单线程独占约束图，无并发写。对应 C++ `void ConstraintGraph::unblockTypeOrPack(TypePackId vertex)`
  /// (`cpp/Analysis/src/ConstraintGraph.cpp:216`)。
  pub unsafe fn unblock_type_or_pack_type_pack_id(&mut self, vertex: TypePackId) {
    self.repair_type_references_type_pack_id(vertex);
    let vertex = follow_type_pack::follow(vertex);
    let _vertex = vertex;

    self.clear_reverse_dependencies_of(BlockedConstraintId::V2(ConstraintId::NULL));
  }
}
