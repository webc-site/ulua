//! C++ `static std::optional<TypePackId> solveFunctionCall(NotNull<TypeFunctionContext>
//! ctx, const Location& location, TypeId fnTy, TypePackId argsPack)`
//! (BuiltinTypeFunctions.cpp:121-192). Resolves a (meta)method overload, unifies a
//! prospective function shape against it, and returns the resulting return pack
//! (instantiating generic substitutions where the overload was generic).
use core::ptr::{null, read};

use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::{polarity::Polarity, unify_result::UnifyResult},
  functions::{
    get_approximate_return_type_for_function_call_type_utils::get_approximate_return_type_for_function_call_type_id_dense_hash_set_type_id as get_approximate_return_type_for_function_call,
    instantiate_2_instantiation_2_alt_b::instantiate_2,
    track_interior_free_type::track_interior_free_type,
    track_interior_free_type_pack::track_interior_free_type_pack,
  },
  records::{
    function_type::FunctionType, overload_resolution::OverloadResolution,
    overload_resolver::OverloadResolver, subtyping::Subtyping,
    type_function_context::TypeFunctionContext, unifier_2::Unifier2,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn solve_function_call(
  ctx: *mut TypeFunctionContext,
  location: Location,
  fn_ty: TypeId,
  args_pack: TypePackId,
) -> Option<TypePackId> {
  let ctx_ref = unsafe { &*ctx };

  // auto resolver = std::make_unique<OverloadResolver>(
  //     ctx->builtins, ctx->arena, ctx->normalizer, ctx->typeFunctionRuntime,
  //     ctx->scope, ctx->ice, ctx->limits, location);
  let subtyping = Subtyping::subtyping_owned(
    ctx_ref.builtins.as_ptr(),
    ctx_ref.arena.as_ptr(),
    ctx_ref.normalizer.as_ptr(),
    ctx_ref.type_function_runtime.as_ptr(),
    ctx_ref.ice.as_ptr(),
  );
  let mut resolver = OverloadResolver {
    builtin_types: ctx_ref.builtins.as_ptr(),
    arena: ctx_ref.arena.as_ptr(),
    normalizer: ctx_ref.normalizer.as_ptr(),
    type_function_runtime: ctx_ref.type_function_runtime.as_ptr(),
    scope: ctx_ref.scope.as_ptr(),
    ice: ctx_ref.ice.as_ptr(),
    limits: unsafe { read(ctx_ref.limits.as_ptr() as *const _) },
    subtyping,
    call_loc: location,
  };

  let mut unique_types: DenseHashSet<TypeId> = DenseHashSet::new(null());
  let resolution: OverloadResolution = resolver.resolve_overload(
    fn_ty,
    args_pack,
    location,
    &mut unique_types as *mut DenseHashSet<TypeId>,
    /* useFreeTypeBounds */ false,
  );

  if resolution.ok.is_empty() && resolution.potential_overloads.is_empty() {
    return None;
  }

  let selected = resolution.get_unambiguous_overload();

  let selected_overload = selected.overload?;

  let mut ret_pack = unsafe {
    (*ctx_ref.arena.as_ptr()).fresh_type_pack(ctx_ref.scope.as_ptr(), Polarity::Positive)
  };
  let prospective_function = unsafe {
    (*ctx_ref.arena.as_ptr()).add_type(FunctionType::function_type_new(
      args_pack, ret_pack, None, false,
    ))
  };

  // FIXME (mirroring C++): we have to bust out the Unifier here.
  let mut unifier = Unifier2::unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter(
        ctx_ref.arena,
        ctx_ref.builtins,
        ctx_ref.scope,
        ctx_ref.ice,
    );

  let unify_result = unifier.unify(selected_overload, prospective_function);

  match unify_result {
    UnifyResult::Ok => {}
    UnifyResult::OccursCheckFailed => return None,
    UnifyResult::TooComplex => return None,
  }

  if !unifier.generic_substitutions.empty() || !unifier.generic_pack_substitutions.empty() {
    let mut subtyping2 = Subtyping::subtyping_owned(
      ctx_ref.builtins.as_ptr(),
      ctx_ref.arena.as_ptr(),
      ctx_ref.normalizer.as_ptr(),
      ctx_ref.type_function_runtime.as_ptr(),
      ctx_ref.ice.as_ptr(),
    );

    let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(null());
    let new_ret_tp = get_approximate_return_type_for_function_call(selected_overload, &mut seen)
      .unwrap_or(unsafe { ctx_ref.builtins.as_ref().error_type_pack });

    // C++ `std::move`s the substitution maps into instantiate2; a clone is the
    // faithful behavioral equivalent (the unifier is dropped at scope end).
    let subst = instantiate_2(
      ctx_ref.arena.as_ptr(),
      unifier.generic_substitutions.clone(),
      unifier.generic_pack_substitutions.clone(),
      &mut subtyping2 as *mut Subtyping,
      ctx_ref.scope.as_ptr(),
      new_ret_tp,
    );

    let subst = subst?;
    ret_pack = subst;
  }

  // After we solve for the instantiated function type of this metamethod, we
  // may have new free types if the metamethod was generic. We capture these so
  // that they can be generalized later and we don't end up with free types in
  // type checking.
  for &ty in &unifier.new_fresh_types {
    track_interior_free_type(ctx_ref.scope.as_ptr(), ty);
  }
  for &tp in &unifier.new_fresh_type_packs {
    track_interior_free_type_pack(ctx_ref.scope.as_ptr(), tp);
  }

  Some(ret_pack)
}
