use crate::records::{json_emitter::JsonEmitter, type_check_log::TypeCheckLog};

pub fn write_json_emitter_type_check_log(emitter: &mut JsonEmitter, log: &TypeCheckLog) {
  let mut o = emitter.write_object();
  o.write_pair("errors", &log.errors);
  o.finish();
}
