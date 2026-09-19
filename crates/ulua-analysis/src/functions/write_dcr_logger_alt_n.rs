use crate::records::{json_emitter::JsonEmitter, scope_snapshot::ScopeSnapshot};

pub fn write_json_emitter_scope_snapshot(emitter: &mut JsonEmitter, snapshot: &ScopeSnapshot) {
  let mut o = emitter.write_object();
  o.write_pair("bindings", &snapshot.bindings);
  o.write_pair("typeBindings", &snapshot.type_bindings);
  o.write_pair("typePackBindings", &snapshot.type_pack_bindings);
  o.write_pair("children", &snapshot.children);
  o.finish();
}
