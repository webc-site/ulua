//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypeFunctionRuntime.cpp:2456:are_equal`
//! Source: `Analysis/src/TypeFunctionRuntime.cpp:2456-2486` (hand-fixed from a
//! handoff batch: the model invented a `::new().get_if::<T>()` API and a
//! nonexistent generic-pack overload — C++ inlines the generic comparison)

use crate::{
  functions::{
    are_equal_type_function_runtime_alt_n::are_equal_are_equal_state_type_function_type_pack_type_function_type_pack,
    are_equal_type_function_runtime_alt_o::are_equal_are_equal_state_type_function_variadic_type_pack_type_function_variadic_type_pack,
  },
  records::{
    are_equal_state::AreEqualState, type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_type_pack_var::TypeFunctionTypePackVar,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
  type_aliases::type_function_type_pack_variant::TypeFunctionTypePackVariantMember,
};

/// C++ `bool areEqual(AreEqualState& seen, const TypeFunctionTypePackVar& lhs,
/// const TypeFunctionTypePackVar& rhs)` — the pack-variant dispatcher.
pub fn are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionTypePackVar,
  rhs: &TypeFunctionTypePackVar,
) -> bool {
  {
    let lb = TypeFunctionTypePack::get_if(&lhs.type_variant);
    let rb = TypeFunctionTypePack::get_if(&rhs.type_variant);
    if let (Some(lb), Some(rb)) = (lb, rb) {
      return are_equal_are_equal_state_type_function_type_pack_type_function_type_pack(
        seen, lb, rb,
      );
    }
  }

  {
    let lv = TypeFunctionVariadicTypePack::get_if(&lhs.type_variant);
    let rv = TypeFunctionVariadicTypePack::get_if(&rhs.type_variant);
    if let (Some(lv), Some(rv)) = (lv, rv) {
      return are_equal_are_equal_state_type_function_variadic_type_pack_type_function_variadic_type_pack(
                seen, lv, rv,
            );
    }
  }

  {
    let lg = TypeFunctionGenericTypePack::get_if(&lhs.type_variant);
    let rg = TypeFunctionGenericTypePack::get_if(&rhs.type_variant);
    if let (Some(lg), Some(rg)) = (lg, rg) {
      return lg.is_named == rg.is_named && lg.name == rg.name;
    }
  }

  let _ = seen;
  false
}
