//! Source: `Analysis/src/OverloadResolver.cpp:457-548` (hand-ported)
//!
//! Test a single FunctionType against an argument list. Reduces type functions
//! and does a proper arity check.
use alloc::vec::Vec;
use core::{
  mem::take,
  ptr::{NonNull, null, null_mut},
};

use ulua_ast::records::location::Location;
use ulua_common::records::{dense_hash_set::DenseHashSet, variant::Variant2};

use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::{
    are_unsatisfied_arguments_optional::are_unsatisfied_arguments_optional,
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    ignore_reasoning_for_return_type::ignore_reasoning_for_return_type,
    reduce_type_functions_type_function::reduce_type_functions,
  },
  records::{
    blocked_type::BlockedType, free_type::FreeType, function_type::FunctionType,
    overload_resolution::OverloadResolution, overload_resolver::OverloadResolver,
    pending_expansion_type::PendingExpansionType, type_error::TypeError,
    type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::{
    error_vec::ErrorVec, type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId,
  },
};
impl OverloadResolver {
  pub fn test_function(
    &mut self,
    result: &mut OverloadResolution,
    fn_ty: TypeId,
    args_pack: TypePackId,
    fn_location: Location,
    unique_types: *mut DenseHashSet<TypeId>,
  ) {
    let fn_ty = follow_type_id(fn_ty);

    // TODO: This seems like the wrong spot to do this check.
    if !get_type_id::<FreeType>(fn_ty).is_none()
      || !get_type_id::<BlockedType>(fn_ty).is_none()
      || !get_type_id::<PendingExpansionType>(fn_ty).is_none()
    {
      // TODO.  Luckily, these constraints are not yet used.
      let constraints = Vec::new();
      result.potential_overloads.push((fn_ty, constraints));
      return;
    }

    if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(fn_ty)
      && tfit.state == TypeFunctionInstanceState::Unsolved
    {
      // TODO.  Luckily, these constraints are not yet used.
      let constraints = Vec::new();
      result.potential_overloads.push((fn_ty, constraints));
      return;
    }

    let Some(ftv) = get_type_id::<FunctionType>(fn_ty) else {
      result.non_functions.push(fn_ty);
      return;
    };

    if !self.is_arity_compatible(args_pack, ftv.arg_types, self.builtin_types) {
      result.arity_mismatches.push(fn_ty);
      return;
    }

    let context = TypeFunctionContext {
      arena: unsafe { NonNull::new_unchecked(self.arena) },
      builtins: unsafe { NonNull::new_unchecked(self.builtin_types) },
      scope: unsafe { NonNull::new_unchecked(self.scope) },
      normalizer: unsafe { NonNull::new_unchecked(self.normalizer) },
      type_function_runtime: unsafe { NonNull::new_unchecked(self.type_function_runtime) },
      ice: unsafe { NonNull::new_unchecked(self.ice) },
      limits: unsafe { NonNull::new_unchecked(&self.limits as *const _ as *mut _) },
      subtyping: unsafe { NonNull::new_unchecked(&mut self.subtyping as *mut _) },
      solver: null_mut(),
      constraint: null(),
      user_func_name: None,
      fresh_instances: Vec::new(),
    };
    let mut context = context;
    let reduce_result = reduce_type_functions(
      fn_ty,
      self.call_loc,
      unsafe { NonNull::new_unchecked(&mut context as *mut _) },
      /*force=*/ true,
    );
    if !reduce_result.errors.is_empty() {
      result
        .incompatible_overloads
        .push((fn_ty, Variant2::V1(reduce_result.errors)));
      return;
    }

    let prospective_function = unsafe {
      (*self.arena).add_type(FunctionType::function_type_new(
        args_pack,
        (*self.builtin_types).any_type_pack,
        None,
        false,
      ))
    };

    self.subtyping.unique_types = unique_types as *const DenseHashSet<TypeId>;
    let scope = self.scope;
    let mut r =
      self
        .subtyping
        .is_subtype_type_id_type_id_not_null_scope(fn_ty, prospective_function, scope);

    // Frustratingly, subtyping does not know about error suppression, so this
    // subtype test will probably fail due to the mismatched return types. Here,
    // we'll prune any SubtypingReasons that have anything to do with the return
    // type.
    ignore_reasoning_for_return_type(&mut r);

    if r.is_subtype {
      if r.assumed_constraints.is_empty() {
        result.ok.push(fn_ty);
      } else {
        result
          .potential_overloads
          .push((fn_ty, take(&mut r.assumed_constraints)));
      }
    } else if !r.generic_bounds_mismatches.is_empty() {
      let mut errors: ErrorVec = Vec::new();
      for gbm in r.generic_bounds_mismatches.iter() {
        errors.push(TypeError::type_error_location_type_error_data(
          fn_location,
          TypeErrorData::GenericBoundsMismatch(gbm.clone()),
        ));
      }
      result
        .incompatible_overloads
        .push((fn_ty, Variant2::V1(errors)));
    } else if are_unsatisfied_arguments_optional(&r.reasoning, args_pack, ftv.arg_types) {
      // Important!  Subtyping doesn't know anything about
      // optional arguments.  If the only reason subtyping
      // failed is because optional arguments were not provided,
      // then this overload is actually okay.
      if r.assumed_constraints.is_empty() {
        result.ok.push(fn_ty);
      } else {
        result
          .potential_overloads
          .push((fn_ty, take(&mut r.assumed_constraints)));
      }
    } else {
      result
        .incompatible_overloads
        .push((fn_ty, Variant2::V0(take(&mut r.reasoning))));
    }
  }
}
