use crate::records::{checkpoint::Checkpoint, constraint_generator::ConstraintGenerator};

/// 当前约束向量长度的快照。`constraints` 为 `Vec<ConstraintPtr>`，只读取其长度。
pub fn checkpoint(cg: &ConstraintGenerator) -> Checkpoint {
  Checkpoint {
    offset: cg.constraints.len(),
  }
}
