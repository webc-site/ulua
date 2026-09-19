//! Source: `Analysis/src/ToString.cpp:1326-1343` (hand-ported)

use ulua_common::fint;

use crate::{
  records::{free_type_pack::FreeTypePack, type_pack_stringifier::TypePackStringifier},
  type_aliases::type_pack_id::TypePackId,
};
impl TypePackStringifier {
  /// C++ `void operator()(TypePackId tp, const FreeTypePack& pack)`.
  pub fn operator_call(&mut self, tp: TypePackId, pack: &FreeTypePack) {
    unsafe {
      (*(*self.state).result).invalid = true;
      if fint::DebugLuauVerboseTypeNames.get() >= 1 {
        (*self.state).emit("free-");
      }
      let name = (*self.state).get_name_type_pack_id(tp);
      (*self.state).emit(name.as_str());

      if fint::DebugLuauVerboseTypeNames.get() >= 1 {
        (*self.state).emit_polarity(pack.polarity);
      }

      if fint::DebugLuauVerboseTypeNames.get() >= 2 {
        (*self.state).emit("-");
        (*self.state).emit_level(pack.scope);
      }

      (*self.state).emit("...");
    }
  }
}
