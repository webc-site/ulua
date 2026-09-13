//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:178:stringifier_state_stringifier_state`
//! Source: `Analysis/src/ToString.cpp:178-190` (hand-ported)

use alloc::string::String;
use core::ptr::{null, null_mut};

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::records::{
  set::Set, stringifier_state::StringifierState, to_string_options::ToStringOptions,
  to_string_result::ToStringResult,
};
impl StringifierState {
  /// C++ `StringifierState(ToStringOptions& opts, ToStringResult& result)`.
  /// The C++ reference members are raw pointers in the record; callers keep
  /// `opts`/`result` alive for the state's lifetime (as in C++).
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn stringifier_state_stringifier_state(
    opts: *mut ToStringOptions,
    result: *mut ToStringResult,
  ) -> Self {
    unsafe {
      let o = &*opts;
      let mut state = StringifierState {
        opts,
        result,
        cycle_names: DenseHashMap::new(null()),
        cycle_tp_names: DenseHashMap::new(null()),
        seen: Set::new(null_mut()),
        // `$$$` is the usedNames tombstone: not a valid name syntactically
        // and short for string comparison reasons.
        used_names: DenseHashSet::new(String::from("$$$")),
        indentation: 0,
        exhaustive: o.exhaustive,
        ignore_synthetic_name: o.ignore_synthetic_name,
        previous_name_index: 0,
      };

      for (_k, v) in o.name_map.types.iter() {
        state.used_names.insert(v.clone());
      }
      for (_k, v) in o.name_map.type_packs.iter() {
        state.used_names.insert(v.clone());
      }

      state
    }
  }
}
