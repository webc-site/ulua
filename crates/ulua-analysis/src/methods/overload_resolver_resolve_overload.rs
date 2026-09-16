//! Node: `cxx:Method:Luau.Analysis:Analysis/src/OverloadResolver.cpp:175:overload_resolver_resolve_overload`
//! Source: `Analysis/src/OverloadResolver.cpp:175-196` (hand-ported)

use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    intersection_type::IntersectionType, overload_resolution::OverloadResolution,
    overload_resolver::OverloadResolver,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl OverloadResolver {
  /// C++ `OverloadResolution resolveOverload(TypeId ty, TypePackId argsPack, Location fnLocation, NotNull<DenseHashSet<TypeId>> uniqueTypes, bool useFreeTypeBounds)`.
  pub fn resolve_overload(
    &mut self,
    ty: TypeId,
    args_pack: TypePackId,
    fn_location: Location,
    unique_types: *mut DenseHashSet<TypeId>,
    _use_free_type_bounds: bool,
  ) -> OverloadResolution {
    let mut result = OverloadResolution {
      ok: Vec::new(),
      non_functions: Vec::new(),
      potential_overloads: Vec::new(),
      incompatible_overloads: Vec::new(),
      arity_mismatches: Vec::new(),
      metamethods: DenseHashSet::new(null_mut()),
    };

    let ty = follow_type_id(ty);

    if let Some(it) = get_type_id::<IntersectionType>(ty) {
      let parts = it.parts.clone();
      for component in parts {
        self.test_function_or_union(&mut result, component, args_pack, fn_location, unique_types);
      }
    } else {
      self.test_function_or_union(&mut result, ty, args_pack, fn_location, unique_types);
    }

    result
  }
}
