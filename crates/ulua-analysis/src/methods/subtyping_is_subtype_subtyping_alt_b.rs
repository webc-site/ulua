//! Faithful port of `Subtyping::isSubtype(subTp, superTp, scope, bindableGenerics)`
//! — the 4-arg pack overload (Analysis/src/Subtyping.cpp:635-639) that constructs
//! an empty `bindableGenericPacks` and delegates to the 5-arg overload.
use alloc::vec::Vec;

use crate::{
  records::{scope::Scope, subtyping::Subtyping, subtyping_result::SubtypingResult},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl Subtyping {
  /// C++:
  /// ```cpp
  /// SubtypingResult Subtyping::isSubtype(TypePackId subTp, TypePackId superTp, NotNull<Scope> scope,
  ///     const std::vector<TypeId>& bindableGenerics)
  /// {
  ///     const std::vector<TypePackId> bindableGenericPacks;
  ///     return isSubtype(subTp, superTp, scope, bindableGenerics, bindableGenericPacks);
  /// }
  /// ```
  pub fn is_subtype_type_pack_id_type_pack_id_not_null_scope_vector_type_id(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    scope: *mut Scope,
    bindable_generics: &[TypeId],
  ) -> SubtypingResult {
    let bindable_generic_packs: Vec<TypePackId> = Vec::new();
    self.is_subtype_type_pack_id_type_pack_id_not_null_scope_vector_type_id_vector_type_pack_id(
      sub_tp,
      super_tp,
      scope,
      bindable_generics,
      &bindable_generic_packs,
    )
  }
}
