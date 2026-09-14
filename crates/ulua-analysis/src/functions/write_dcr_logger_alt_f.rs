use ulua_ast::records::location::Location;

use crate::records::json_emitter::JsonEmitter;

/// `void write(JsonEmitter& emitter, const Location& location)`
/// (`Analysis/src/DcrLogger.cpp:37-45`).
pub fn write_json_emitter_location(emitter: &mut JsonEmitter, location: &Location) {
  let mut a = emitter.write_array();
  a.write_value(location.begin.line);
  a.write_value(location.begin.column);
  a.write_value(location.end.line);
  a.write_value(location.end.column);
  a.finish();
}
