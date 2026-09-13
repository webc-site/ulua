//! Source: `Analysis/include/Luau/JsonEmitter.h` (lines 227-234, hand-ported)
//!
//! C++ template:
//! ```cpp
//! template<typename T>
//! void write(JsonEmitter& emitter, const std::optional<T>& v)
//! {
//!     if (v.has_value())
//!         write(emitter, *v);
//!     else
//!         emitter.writeRaw("null");
//! }
//! ```

use crate::{methods::object_emitter_write_pair::WriteJson, records::json_emitter::JsonEmitter};

pub fn write<T: WriteJson>(emitter: &mut JsonEmitter, v: &Option<T>) {
  match v {
    Some(value) => value.write_json(emitter),
    None => emitter.write_raw_string_view("null"),
  }
}

/// `write(JsonEmitter&, const std::optional<T>&)` exposed through `WriteJson` so
/// an optional can be a key value.
impl<T: WriteJson> WriteJson for Option<T> {
  fn write_json(&self, emitter: &mut JsonEmitter) {
    write(emitter, self);
  }
}
