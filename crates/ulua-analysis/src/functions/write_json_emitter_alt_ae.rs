use ulua_common::functions::format::format;

use crate::records::json_emitter::JsonEmitter;

pub fn write_json_emitter_string_view(emitter: &mut JsonEmitter, sv: &str) {
  emitter.write_raw_string_view("\"");

  for c in sv.chars() {
    match c {
      '"' => emitter.write_raw_string_view("\\\""),
      '\\' => emitter.write_raw_string_view("\\\\"),
      '\n' => emitter.write_raw_string_view("\\n"),
      c if (c as u32) < 32 => {
        let s = format(format_args!("\\u{:04x}", c as u32));
        emitter.write_raw_string_view(&s);
      }
      c => {
        let mut buf = [0u8; 4];
        emitter.write_raw_string_view(c.encode_utf8(&mut buf));
      }
    }
  }

  emitter.write_raw_string_view("\"");
}
