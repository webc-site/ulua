use crate::records::json_emitter::JsonEmitter;

pub fn write_json_emitter_bool(emitter: &mut JsonEmitter, b: bool) {
  if b {
    emitter.write_raw_string_view("true");
  } else {
    emitter.write_raw_string_view("false");
  }
}
