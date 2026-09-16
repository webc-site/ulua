use crate::records::json_emitter::JsonEmitter;

pub fn write_json_emitter_f64(emitter: &mut JsonEmitter, d: f64) {
  emitter.write_raw_string_view(d.to_string().as_str());
}
