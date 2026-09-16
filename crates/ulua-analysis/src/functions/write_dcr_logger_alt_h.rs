use crate::records::{binding_snapshot::BindingSnapshot, json_emitter::JsonEmitter};

pub fn write_json_emitter_binding_snapshot(emitter: &mut JsonEmitter, snapshot: &BindingSnapshot) {
  let mut o = emitter.write_object();
  o.write_pair("typeId", &snapshot.type_id);
  o.write_pair("typeString", &snapshot.type_string);
  o.write_pair("location", snapshot.location);
  o.finish();
}
