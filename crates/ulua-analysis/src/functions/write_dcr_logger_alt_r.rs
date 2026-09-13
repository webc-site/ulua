use crate::records::{constraint_step_snapshot::ConstraintStepSnapshot, json_emitter::JsonEmitter};

pub fn write_json_emitter_constraint_step_snapshot(
  emitter: &mut JsonEmitter,
  snapshot: &ConstraintStepSnapshot,
) {
  let mut o = emitter.write_object();
  o.write_pair("type", "constraint");
  o.write_pair("currentConstraint", snapshot.current_constraint);
  o.write_pair("forced", snapshot.forced);
  o.write_pair("unsolvedConstraints", &snapshot.unsolved_constraints);
  o.write_pair("rootScope", &snapshot.root_scope);
  o.write_pair("typeStrings", &snapshot.type_strings);
  o.finish();
}
