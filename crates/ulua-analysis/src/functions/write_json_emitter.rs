use alloc::{string::String, vec::Vec};

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};

use crate::{
  functions::write_dcr_logger::write_json_emitter_dense_hash_map_k_v,
  methods::object_emitter_write_pair::WriteJson, records::json_emitter::JsonEmitter,
  type_aliases::collections::HashMap,
};

// Source: `Analysis/include/Luau/JsonEmitter.h` (lines 212-221, hand-ported)
//
// C++ template:
// ```cpp
// template<typename T>
// void write(JsonEmitter& emitter, const std::vector<T>& vec)
// {
//     ArrayEmitter a = emitter.writeArray();
//     for (const T& value : vec)
//         a.writeValue(value);
//     a.finish();
// }
// ```

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

const CONTROL_ESCAPES: [&str; 32] = [
  "\\u0000", "\\u0001", "\\u0002", "\\u0003", "\\u0004", "\\u0005", "\\u0006", "\\u0007", "\\b",
  "\\t", "\\n", "\\u000b", "\\f", "\\r", "\\u000e", "\\u000f", "\\u0010", "\\u0011", "\\u0012",
  "\\u0013", "\\u0014", "\\u0015", "\\u0016", "\\u0017", "\\u0018", "\\u0019", "\\u001a",
  "\\u001b", "\\u001c", "\\u001d", "\\u001e", "\\u001f",
];

pub fn write_string(sv: &str, mut write_raw: impl FnMut(&str)) {
  write_raw("\"");
  let mut start = 0;
  for (index, byte) in sv.bytes().enumerate() {
    let escaped = match byte {
      b'"' => "\\\"",
      b'\\' => "\\\\",
      0..=31 => CONTROL_ESCAPES[usize::from(byte)],
      _ => continue,
    };
    if start != index {
      write_raw(&sv[start..index]);
    }
    write_raw(escaped);
    start = index + 1;
  }
  if start != sv.len() {
    write_raw(&sv[start..]);
  }
  write_raw("\"");
}

pub fn write_json_emitter_string_view(emitter: &mut JsonEmitter, sv: &str) {
  write_string(sv, |part| emitter.write_raw_string_view(part));
}

pub fn write_json_emitter_nullptr_t(emitter: &mut JsonEmitter, _null: *const ()) {
  emitter.write_raw_string_view("null");
}

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

pub fn write_json_emitter_bool(emitter: &mut JsonEmitter, b: bool) {
  if b {
    emitter.write_raw_string_view("true");
  } else {
    emitter.write_raw_string_view("false");
  }
}

pub fn write_json_emitter_f64(emitter: &mut JsonEmitter, d: f64) {
  emitter.write_raw_string_view(d.to_string().as_str());
}
