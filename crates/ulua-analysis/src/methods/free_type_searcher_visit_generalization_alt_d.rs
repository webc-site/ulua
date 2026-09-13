use core::ffi::c_void;

use crate::{
  methods::free_type_searcher_visit_generalization_alt_c::searcher_traverse_type_pack_id,
  records::{free_type_searcher::FreeTypeSearcher, function_type::FunctionType},
  type_aliases::type_id::TypeId,
};
impl FreeTypeSearcher {
  /// C++ `bool FreeTypeSearcher::visit(TypeId ty, const FunctionType& ft)`
  /// (Generalization.cpp:179-196). Flips polarity across the parameter pack:
  /// arguments are traversed in inverted polarity (contravariant), return
  /// types in the current polarity (covariant). Marks the body as being
  /// within a function for the duration. Returns `false` because it performs
  /// its own traversal of the argument and return packs.
  pub fn visit_type_id_function_type(&mut self, ty: TypeId, ft: &FunctionType) -> bool {
    if unsafe { (*self.cached_types).contains(&ty) }
      || self.seen_with_current_polarity(ty as *const c_void)
    {
      return false;
    }

    let old_value = self.is_within_function;
    self.is_within_function = true;

    self.flip();
    searcher_traverse_type_pack_id(self, ft.arg_types);
    self.flip();

    searcher_traverse_type_pack_id(self, ft.ret_types);

    self.is_within_function = old_value;

    false
  }
}
