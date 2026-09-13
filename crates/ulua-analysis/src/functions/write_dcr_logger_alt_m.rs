use crate::records::{
  constraint_generation_log::ConstraintGenerationLog, json_emitter::JsonEmitter,
  object_emitter::ObjectEmitter,
};

pub fn write_json_emitter_constraint_generation_log(
  emitter: &mut JsonEmitter,
  log: &ConstraintGenerationLog,
) {
  let mut o: ObjectEmitter = emitter.write_object();
  o.write_pair("source", &log.source);
  o.write_pair("errors", &log.errors);
  o.write_pair("exprTypeLocations", &log.expr_type_locations);
  o.write_pair("annotationTypeLocations", &log.annotation_type_locations);
  o.finish();
}
