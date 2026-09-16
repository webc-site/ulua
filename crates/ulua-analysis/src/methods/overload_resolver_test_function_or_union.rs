//! Source: `Analysis/src/OverloadResolver.cpp:550-593` (hand-ported)
use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::records::location::Location;
use ulua_common::{
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_set::DenseHashSet, variant::Variant2},
};

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    cannot_call_non_function::CannotCallNonFunction, overload_resolution::OverloadResolution,
    overload_resolver::OverloadResolver, type_error::TypeError, union_type::UnionType,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId},
};
impl OverloadResolver {
  pub fn test_function_or_union(
    &mut self,
    result: &mut OverloadResolution,
    fn_ty: TypeId,
    args_pack: TypePackId,
    fn_location: Location,
    unique_types: *mut DenseHashSet<TypeId>,
  ) {
    let fn_ty = follow_type_id(fn_ty);
    LUAU_ASSERT!(fn_ty == follow_type_id(fn_ty));

    if let Some(ut) = get_type_id::<UnionType>(fn_ty) {
      // A union of functions is a valid overload iff every type within it is a valid overload.

      let mut inner_result = OverloadResolution {
        ok: Vec::new(),
        non_functions: Vec::new(),
        potential_overloads: Vec::new(),
        incompatible_overloads: Vec::new(),
        arity_mismatches: Vec::new(),
        metamethods: DenseHashSet::new(null_mut()),
      };
      let options = ut.options.clone();
      let mut count: usize = 0;
      for t in options {
        count += 1;
        self.test_function_or_call_metamethod(
          &mut inner_result,
          t,
          args_pack,
          fn_location,
          unique_types,
        );
      }

      if count == inner_result.ok.len() {
        result.ok.push(fn_ty);
      } else if count == inner_result.ok.len() + inner_result.potential_overloads.len() {
        let mut all_constraints = Vec::new();
        for (_t, constraints) in inner_result.potential_overloads.iter() {
          all_constraints.extend(constraints.iter().cloned());
        }

        result.potential_overloads.push((fn_ty, all_constraints));
      } else {
        // FIXME: We should probably report something better here, but it's
        // important for type checking that we include this.
        let errors = alloc::vec![TypeError::type_error_location_type_error_data(
          fn_location,
          TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: fn_ty }),
        )];
        result
          .incompatible_overloads
          .push((fn_ty, Variant2::V1(errors)));
      }
    } else {
      self.test_function_or_call_metamethod(result, fn_ty, args_pack, fn_location, unique_types);
    }
  }
}
