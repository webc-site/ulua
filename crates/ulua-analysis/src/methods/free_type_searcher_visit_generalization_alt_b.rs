use core::ffi::c_void;

use crate::{
  functions::subsumes_scope::subsumes,
  records::{free_type::FreeType, free_type_searcher::FreeTypeSearcher},
  type_aliases::type_id::TypeId,
};
impl FreeTypeSearcher {
  /// C++ `bool FreeTypeSearcher::visit(TypeId ty, const FreeType& ft)`
  /// (Generalization.cpp:100-117).
  pub fn visit_type_id_free_type(&mut self, ty: TypeId, ft: &FreeType) -> bool {
    if !subsumes(self.scope, ft.scope) {
      return true;
    }

    // GeneralizationParams<TypeId>& params = types[ty]; ++params.useCount;
    self.types.get_or_default(ty).use_count += 1;

    if unsafe { (*self.cached_types).contains(&ty) }
      || self.seen_with_current_polarity(ty as *const c_void)
    {
      return false;
    }

    let is_within_function = self.is_within_function;
    let polarity = self.polarity;
    let params = self.types.get_or_default(ty);

    if !is_within_function {
      params.found_outside_functions = true;
    }

    params.polarity |= polarity;

    true
  }
}
