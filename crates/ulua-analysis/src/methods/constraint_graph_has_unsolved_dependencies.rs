use crate::{
  records::{
    blocked_constraint_registry::resolve_constraint, constraint_graph::ConstraintGraph,
    primitive_type_constraint::PrimitiveTypeConstraint,
  },
  type_aliases::{blocked_constraint_id::BlockedConstraintId, constraint_v::ConstraintVMember},
};

impl ConstraintGraph {
  pub fn has_unsolved_dependencies(&mut self, vertex: BlockedConstraintId) -> bool {
    let deps = self.find_dependency_list(vertex.clone());

    if let Some(c) = vertex.get_if_2() {
      // §2：V2 分支即 `ConstraintId` 句柄，读回节点走 `resolve_constraint`
      // （指向本次判定期间存活、对齐良好的 `Constraint`）；本分支仅以只读
      // 借用做 `PrimitiveTypeConstraint::get_if` 的变体判定，不改写该节点。
      let Some(constraint) = resolve_constraint(*c) else {
        return unsafe { deps.as_ref().size() > 0 };
      };
      if PrimitiveTypeConstraint::get_if(&constraint.c).is_some() {
        // Safety: `deps` 是 `find_dependency_list` 返回的 `NonNull<ConstraintList>`，
        // 指向 self.constraint_lists bump arena 中地址稳定的存活节点；NonNull 构造期保证
        // 非空，且 find_dependency_list 的 &mut 借用已随其返回结束，此处只读 size 无别名。
        return unsafe { deps.as_ref().size() > 1 };
      }
    }

    // Safety: 同上——`deps` 非空且指向地址稳定的存活 ConstraintList，只读 size 借用。
    unsafe { deps.as_ref().size() > 0 }
  }
}
