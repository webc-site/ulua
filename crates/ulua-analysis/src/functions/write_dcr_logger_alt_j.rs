//! Source: `Analysis/src/DcrLogger.cpp` (lines 72-79, faithful port)
//!
//! C++ template:
//! ```cpp
//! template<typename K, typename V>
//! void write(JsonEmitter& emitter, const DenseHashMap<const K*, V>& map)
//! {
//!     ObjectEmitter o = emitter.writeObject();
//!     for (const auto& [k, v] : map)
//!         o.writePair(toPointerId(k), v);
//!     o.finish();
//! }
//! ```
//!
//! Writes a pointer-keyed `DenseHashMap` as a JSON object whose member names are
//! each key's pointer id (`toPointerId`) and whose values are the mapped `V`.

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};

use crate::{
  functions::to_pointer_id_dcr_logger::to_pointer_id,
  methods::object_emitter_write_pair::WriteJson, records::json_emitter::JsonEmitter,
};

pub fn write_json_emitter_dense_hash_map_k_v<K, V: WriteJson + DenseDefault>(
  emitter: &mut JsonEmitter,
  map: &DenseHashMap<*const K, V>,
) {
  let mut o = emitter.write_object();

  for (k, v) in map.iter() {
    o.write_pair(&to_pointer_id(*k), v);
  }

  o.finish();
}
