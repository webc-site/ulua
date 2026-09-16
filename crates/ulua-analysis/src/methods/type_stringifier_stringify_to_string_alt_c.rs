//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:454:type_stringifier_stringify`
//! Source: `Analysis/src/ToString.cpp:454-500` (hand-ported)

use crate::{
  functions::{follow_type_pack::follow_type_pack_id, get_type_pack::get, is_empty::is_empty},
  records::{type_pack::TypePack, type_stringifier::TypeStringifier},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeStringifier {
  /// C++ `void stringify(const std::vector<TypeId>& types, const std::vector<TypePackId>& typePacks)`.
  pub fn stringify_vector_type_id_vector_type_pack_id(
    &mut self,
    types: &[TypeId],
    type_packs: &[TypePackId],
  ) {
    unsafe {
      if types.is_empty() && type_packs.is_empty() {
        return;
      }

      if !types.is_empty() || !type_packs.is_empty() {
        (*self.state).emit("<");
      }

      let mut first = true;

      for &ty in types.iter() {
        if !first {
          (*self.state).emit(", ");
        }
        first = false;

        self.stringify_type_id(ty);
      }

      let single_tp = type_packs.len() == 1;

      for &tp in type_packs.iter() {
        if is_empty(tp) && single_tp {
          continue;
        }

        if !first {
          (*self.state).emit(", ");
        } else {
          first = false;
        }

        let mut wrap = !single_tp && !get::<TypePack>(follow_type_pack_id(tp)).is_none();

        wrap &= !is_empty(tp);

        if wrap {
          (*self.state).emit("(");
        }

        self.stringify_type_pack_id(tp);

        if wrap {
          (*self.state).emit(")");
        }
      }

      if !types.is_empty() || !type_packs.is_empty() {
        (*self.state).emit(">");
      }
    }
  }
}
