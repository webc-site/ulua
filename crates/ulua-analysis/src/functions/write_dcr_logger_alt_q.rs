use crate::records::{boundary_snapshot::BoundarySnapshot, json_emitter::JsonEmitter};

pub fn write_json_emitter_boundary_snapshot(
  emitter: &mut JsonEmitter,
  snapshot: &BoundarySnapshot,
) {
  let mut o = emitter.write_object();
  o.write_pair("rootScope", &snapshot.root_scope);
  o.write_pair("unsolvedConstraints", &snapshot.unsolved_constraints);
  o.write_pair("typeStrings", &snapshot.type_strings);
  o.finish();
}
