use crate::records::{error_snapshot::ErrorSnapshot, json_emitter::JsonEmitter};

pub fn write_json_emitter_error_snapshot(emitter: &mut JsonEmitter, snapshot: &ErrorSnapshot) {
  let mut o = emitter.write_object();
  o.write_pair("message", &snapshot.message);
  o.write_pair("location", snapshot.location);
  o.finish();
}
