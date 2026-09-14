use crate::records::{json_emitter::JsonEmitter, type_binding_snapshot::TypeBindingSnapshot};

pub fn write_json_emitter_type_binding_snapshot(
  emitter: &mut JsonEmitter,
  snapshot: &TypeBindingSnapshot,
) {
  let mut o = emitter.write_object();
  o.write_pair("typeId", &snapshot.type_id);
  o.write_pair("typeString", &snapshot.type_string);
  o.finish();
}
