use crate::type_aliases::{
  def_id_control_flow_graph::DefId, refinement_id_control_flow_graph::RefinementId,
};

#[derive(Debug, Clone)]
pub struct Refine {
  pub definition: DefId,
  /// 指向 `RefinementArena`（bump arena，地址稳定）存活节点的 NotNull 句柄，
  /// 由 `emitRefineInstruction` 构造期接线，随 CFG 与 arena 同生命周期。
  pub prop: RefinementId,
}
