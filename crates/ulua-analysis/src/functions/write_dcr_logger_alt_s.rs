use crate::records::{generalize_step_snapshot::GeneralizeStepSnapshot, json_emitter::JsonEmitter};

pub fn write_json_emitter_generalize_step_snapshot(
  emitter: &mut JsonEmitter,
  eg: &GeneralizeStepSnapshot,
) {
  let mut o = emitter.write_object();
  o.write_pair("type", "generalize");
  o.write_pair("before", &eg.before);
  o.write_pair("after", &eg.after);
  o.write_pair("unsolvedConstraints", &eg.unsolved_constraints);
  o.write_pair("rootScope", &eg.root_scope);
  o.write_pair("typeStrings", &eg.type_strings);
  o.finish();
}
