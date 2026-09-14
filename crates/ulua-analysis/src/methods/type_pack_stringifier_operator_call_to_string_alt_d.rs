//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1298:type_pack_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:1298-1324` (hand-ported)

use ulua_common::FInt;

use crate::{
  records::{generic_type_pack::GenericTypePack, type_pack_stringifier::TypePackStringifier},
  type_aliases::type_pack_id::TypePackId,
};
impl TypePackStringifier {
  /// C++ `void operator()(TypePackId tp, const GenericTypePack& pack)`.
  pub fn operator_call_2(&mut self, tp: TypePackId, pack: &GenericTypePack) {
    unsafe {
      if FInt::DebugLuauVerboseTypeNames.get() >= 1 {
        (*self.state).emit("gen-");
      }

      if pack.explicit_name {
        (*self.state).used_names.insert(pack.name.clone());
        *(*(*self.state).opts).name_map.type_packs.get_or_insert(tp) = pack.name.clone();
        (*self.state).emit(pack.name.as_str());
      } else {
        let name = (*self.state).get_name_type_pack_id(tp);
        (*self.state).emit(name.as_str());
      }

      if FInt::DebugLuauVerboseTypeNames.get() >= 1 {
        (*self.state).emit_polarity(pack.polarity);
      }

      if FInt::DebugLuauVerboseTypeNames.get() >= 2 {
        (*self.state).emit("-");
        (*self.state).emit_level(pack.scope);
      }

      (*self.state).emit("...");
    }
  }
}
