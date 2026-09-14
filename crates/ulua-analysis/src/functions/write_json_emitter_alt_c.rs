//! Source: `Analysis/include/Luau/JsonEmitter.h` (lines 236-245, hand-ported)
//!
//! C++ template:
//! ```cpp
//! template<typename T>
//! void write(JsonEmitter& emitter, const std::unordered_map<std::string, T>& map)
//! {
//!     ObjectEmitter o = emitter.writeObject();
//!     for (const auto& [k, v] : map)
//!         o.writePair(k, v);
//!     o.finish();
//! }
//! ```
//!
//! DcrLogger stores these maps as `DenseHashMap` / `std::unordered_map` (ported
//! to `std::collections::HashMap`); both string-keyed forms write as objects
//! keyed by the (string) key, exactly as the C++ unordered_map overload.

use std::collections::HashMap;
extern crate alloc;

use alloc::string::String;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};

use crate::{
  functions::write_dcr_logger_alt_j::write_json_emitter_dense_hash_map_k_v,
  methods::object_emitter_write_pair::WriteJson, records::json_emitter::JsonEmitter,
};

pub fn write_json_emitter_unordered_map_string_t<T: WriteJson + DenseDefault>(
  emitter: &mut JsonEmitter,
  map: &DenseHashMap<String, T>,
) {
  let mut o = emitter.write_object();

  for (k, v) in map.iter() {
    o.write_pair(k.as_str(), v);
  }

  o.finish();
}

/// `write(JsonEmitter&, const std::unordered_map<std::string, T>&)` for a
/// string-keyed `DenseHashMap`.
impl<T: WriteJson + DenseDefault> WriteJson for DenseHashMap<String, T> {
  fn write_json(&self, emitter: &mut JsonEmitter) {
    write_json_emitter_unordered_map_string_t(emitter, self);
  }
}

/// Same overload for the `std::unordered_map<std::string, T>` ported to the std
/// `HashMap` (used by `ScopeSnapshot::bindings` etc.).
impl<T: WriteJson> WriteJson for HashMap<String, T> {
  fn write_json(&self, emitter: &mut JsonEmitter) {
    let mut o = emitter.write_object();
    for (k, v) in self.iter() {
      o.write_pair(k.as_str(), v);
    }
    o.finish();
  }
}

/// Pointer-keyed `DenseHashMap` (e.g. `unsolvedConstraints`, `typeStrings`):
/// DcrLogger writes these through its own `write` overload, which keys the
/// object by each key's pointer id.
impl<K, V: WriteJson + DenseDefault> WriteJson for DenseHashMap<*const K, V> {
  fn write_json(&self, emitter: &mut JsonEmitter) {
    write_json_emitter_dense_hash_map_k_v(emitter, self);
  }
}
