use core::ffi::c_void;

use crate::{
  functions::subsumes_scope::subsumes,
  records::{free_type_pack::FreeTypePack, free_type_searcher::FreeTypeSearcher},
  type_aliases::type_pack_id::TypePackId,
};
impl FreeTypeSearcher {
  /// C++ `bool FreeTypeSearcher::visit(TypePackId tp, const FreeTypePack& ftp)`
  /// (Generalization.cpp:203-220).
  pub fn visit_type_pack_id_free_type_pack(&mut self, tp: TypePackId, ftp: &FreeTypePack) -> bool {
    if self.seen_with_current_polarity(tp as *const c_void) {
      return false;
    }

    if !subsumes(self.scope, ftp.scope) {
      return true;
    }

    // GeneralizationParams<TypePackId>& params = typePacks[tp]; ++params.useCount;
    self.type_packs.get_or_default(tp).use_count += 1;

    let is_within_function = self.is_within_function;
    let polarity = self.polarity;
    let params = self.type_packs.get_or_default(tp);

    if !is_within_function {
      params.found_outside_functions = true;
    }

    params.polarity |= polarity;

    true
  }
}
