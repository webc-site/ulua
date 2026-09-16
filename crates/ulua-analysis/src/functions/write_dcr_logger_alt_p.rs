use crate::records::{constraint_snapshot::ConstraintSnapshot, json_emitter::JsonEmitter};

pub fn write_json_emitter_constraint_snapshot(
  emitter: &mut JsonEmitter,
  snapshot: &ConstraintSnapshot,
) {
  let mut o = emitter.write_object();
  o.write_pair("stringification", &snapshot.stringification);
  o.write_pair("location", snapshot.location);
  o.write_pair("blocks", &snapshot.blocks);
  o.finish();
}
