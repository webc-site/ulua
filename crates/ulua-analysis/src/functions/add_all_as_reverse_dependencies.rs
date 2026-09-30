use super::add_all_as_dependencies::add_all_as_edges;
use crate::records::{
  checkpoint::Checkpoint, constraint::Constraint, constraint_generator::ConstraintGenerator,
};

/// 对应 C++ `addAllAsReverseDependencies`（ConstraintGenerator.cpp:157-174）：对
/// `[start,end)` 区间内每条约束 C 建边 `target → C`，阻塞 target 的分发。
pub fn add_all_as_reverse_dependencies(
  start: Checkpoint,
  end: Checkpoint,
  cg: &ConstraintGenerator,
  target: *mut Constraint,
) {
  add_all_as_edges(start, end, cg, target, true);
}
