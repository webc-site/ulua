//! Source: `Analysis/src/OverloadResolver.cpp:175-196` (hand-ported)

use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{begin_type::begin_intersection_type, follow_type, get_type},
  records::{
    intersection_type::IntersectionType, overload_resolution::OverloadResolution,
    overload_resolver::OverloadResolver,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl OverloadResolver<'_> {
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
      metamethods: DenseHashSet::default(),
    };

    let ty = follow_type::follow(ty);

    if let Some(it) = get_type::get::<IntersectionType>(ty) {
      // C++ `for (TypeId component : it)`——IntersectionTypeIterator 展平
      // 嵌套 intersection 并 follow,裸遍历 parts 会漏掉嵌套成员。
      for component in begin_intersection_type(it) {
        self.test_function_or_union(&mut result, component, args_pack, fn_location, unique_types);
      }
    } else {
      self.test_function_or_union(&mut result, ty, args_pack, fn_location, unique_types);
    }

    result
  }
}
