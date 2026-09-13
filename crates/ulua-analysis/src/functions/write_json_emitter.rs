//! Source: `Analysis/include/Luau/JsonEmitter.h` (lines 212-221, hand-ported)
//!
//! C++ template:
//! ```cpp
//! template<typename T>
//! void write(JsonEmitter& emitter, const std::vector<T>& vec)
//! {
//!     ArrayEmitter a = emitter.writeArray();
//!     for (const T& value : vec)
//!         a.writeValue(value);
//!     a.finish();
//! }
//! ```

extern crate alloc;

use alloc::vec::Vec;

use crate::{methods::object_emitter_write_pair::WriteJson, records::json_emitter::JsonEmitter};

pub fn write_json_emitter_vector_t<T: WriteJson>(emitter: &mut JsonEmitter, vec: &Vec<T>) {
  let mut a = emitter.write_array();

  for value in vec {
    a.write_value(value);
  }

  a.finish();
}

/// `write(JsonEmitter&, const std::vector<T>&)` exposed through the `WriteJson`
/// overload set so a `std::vector<T>` can be a key value (`writePair`/`writeValue`).
impl<T: WriteJson> WriteJson for Vec<T> {
  fn write_json(&self, emitter: &mut JsonEmitter) {
    write_json_emitter_vector_t(emitter, self);
  }
}

/// Slices behave like vectors for the overload set; useful for `&[T]` values.
impl<T: WriteJson> WriteJson for [T] {
  fn write_json(&self, emitter: &mut JsonEmitter) {
    let mut a = emitter.write_array();
    for value in self {
      a.write_value(value);
    }
    a.finish();
  }
}
