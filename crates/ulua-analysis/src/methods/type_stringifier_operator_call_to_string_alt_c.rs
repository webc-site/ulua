//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:547:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:547-568` (hand-ported)

use ulua_common::FInt;

use crate::{
  records::{generic_type::GenericType, type_stringifier::TypeStringifier},
  type_aliases::type_id::TypeId,
};
impl TypeStringifier {
  /// C++ `void operator()(TypeId ty, const GenericType& gtv)`.
  pub fn operator_call_3(&mut self, ty: TypeId, gtv: &GenericType) {
    unsafe {
      if FInt::DebugLuauVerboseTypeNames.get() >= 1 {
        (*self.state).emit("gen-");
      }

      if gtv.explicit_name {
        (*self.state).used_names.insert(gtv.name.clone());
        *(*(*self.state).opts).name_map.types.get_or_insert(ty) = gtv.name.clone();
        (*self.state).emit(gtv.name.as_str());
      } else {
        let name = (*self.state).get_name_type_id(ty);
        (*self.state).emit(name.as_str());
      }

      if FInt::DebugLuauVerboseTypeNames.get() >= 1 {
        (*self.state).emit_polarity(gtv.polarity);
      }

      if FInt::DebugLuauVerboseTypeNames.get() >= 2 {
        (*self.state).emit("-");
        (*self.state).emit_level(gtv.scope);
      }
    }
  }
}
