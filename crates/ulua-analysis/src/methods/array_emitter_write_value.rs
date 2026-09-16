//! Source: `Analysis/include/Luau/JsonEmitter.h` (lines 190-200, hand-ported)
//!
//! C++ template:
//! ```cpp
//! template<typename T>
//! void writeValue(T value)
//! {
//!     if (finished) return;
//!     emitter->writeComma();
//!     write(*emitter, value);
//! }
//! ```

use crate::{methods::object_emitter_write_pair::WriteJson, records::array_emitter::ArrayEmitter};

impl ArrayEmitter {
  /// `writeValue(T value)`
  pub fn write_value<T: WriteJson>(&mut self, value: T) {
    if self.finished {
      return;
    }

    let emitter = unsafe { &mut *self.emitter };
    emitter.write_comma();
    // write(*emitter, value)
    value.write_json(emitter);
  }
}
