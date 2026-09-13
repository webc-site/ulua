//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:232:stringifier_state_get_name`
//! Source: `Analysis/src/ToString.cpp:232-252` (hand-ported)

use alloc::string::String;

use crate::{
  functions::generate_name::generate_name, records::stringifier_state::StringifierState,
  type_aliases::type_pack_id::TypePackId,
};

impl StringifierState {
  /// C++ `std::string getName(TypePackId ty)`.
  pub fn get_name_type_pack_id(&mut self, ty: TypePackId) -> String {
    unsafe {
      let opts = &mut *self.opts;
      let s = opts.name_map.type_packs.size();
      {
        let n = opts.name_map.type_packs.get_or_insert(ty);
        if !n.is_empty() {
          return n.clone();
        }
      }

      for count in 0..256i32 {
        let candidate = generate_name((self.previous_name_index + count) as usize);
        if !self.used_names.contains(&candidate) {
          self.previous_name_index += count;
          self.used_names.insert(candidate.clone());
          *opts.name_map.type_packs.get_or_insert(ty) = candidate.clone();
          return candidate;
        }
      }

      generate_name(s)
    }
  }
}
