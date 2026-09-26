use crate::{
  functions::{
    snapshot_scope::snapshot_scope, snapshot_type_strings::snapshot_type_strings,
    to_string_to_string::to_string_constraint_to_string_options,
  },
  records::{
    boundary_snapshot::BoundarySnapshot, constraint::Constraint,
    constraint_snapshot::ConstraintSnapshot, dcr_logger::DcrLogger, scope::Scope,
  },
};

impl DcrLogger {
  pub fn capture_boundary_state(
    &mut self,
    target: &mut BoundarySnapshot,
    root_scope: &Scope,
    unsolved_constraints: &[*const Constraint],
  ) {
    target.root_scope = snapshot_scope(root_scope, &mut self.opts);
    target.unsolved_constraints.clear();

    for &c in unsolved_constraints.iter() {
      // Safety: `unsolved_constraints` 由 solver 传入，其 `*const Constraint` 元素均指向
      // 本次求解期间存活、由约束 arena 持有的 `Constraint` 节点；此处仅取只读引用交给
      // `to_string_constraint_*` 做字符串化，不写入、不释放该节点。
      let constraint_str = to_string_constraint_to_string_options(unsafe { &*c }, &mut self.opts);
      // Safety: 同上——`c` 指向存活的 `Constraint`，`location` 是 Copy 字段，只读拷贝。
      let location = unsafe { (*c).location };
      let blocks = self.snapshot_blocks(c);
      let snapshot = ConstraintSnapshot {
        stringification: constraint_str,
        location,
        blocks,
      };
      *target.unsolved_constraints.get_or_insert(c) = snapshot;
    }

    let Self {
      generation_log,
      opts,
      ..
    } = self;
    snapshot_type_strings(
      &generation_log.expr_type_locations,
      &generation_log.annotation_type_locations,
      &mut target.type_strings,
      opts,
    );
  }
}
