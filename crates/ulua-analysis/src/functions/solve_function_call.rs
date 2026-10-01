//! C++ `static std::optional<TypePackId> solveFunctionCall(NotNull<TypeFunctionContext>
//! ctx, const Location& location, TypeId fnTy, TypePackId argsPack)`
//! (BuiltinTypeFunctions.cpp:121-192). Resolves a (meta)method overload, unifies a
//! prospective function shape against it, and returns the resulting return pack
//! (instantiating generic substitutions where the overload was generic).

use std::mem::take;

use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::{polarity::Polarity, unify_result::UnifyResult},
  functions::{
    get_approximate_return_type_for_function_call_type_utils::get_approximate_return_type_for_function_call_type_id_dense_hash_set_type_id as get_approximate_return_type_for_function_call,
    instantiate_2_instantiation_2::instantiate_2_type_pack,
    track_interior_free_type::track_interior_free_type,
    track_interior_free_type_pack::track_interior_free_type_pack,
  },
  records::{
    arena_handle::{Handle, alias, alias_ref},
    function_type::FunctionType,
    overload_resolution::OverloadResolution,
    overload_resolver::OverloadResolver,
    subtyping::Subtyping,
    type_function_context::TypeFunctionContext,
    unifier_2::Unifier2,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// 对应 C++ `solveFunctionCall(TypeFunctionContext&, ...)` 的引用契约：`ctx`
/// 为共享借用，其全部成员（builtins/arena/normalizer/typeFunctionRuntime/
/// scope/ice/limits）由 `TypeFunctionContext` 构造期接线为非空且存活于整个
/// 求解会话，体内逐处 NonNull 重建已收口在 `unsafe {}` 并各自论证。
pub fn solve_function_call(
  ctx: &TypeFunctionContext,
  location: Location,
  fn_ty: TypeId,
  args_pack: TypePackId,
) -> Option<TypePackId> {
  // auto resolver = std::make_unique<OverloadResolver>(
  //     ctx->builtins, ctx->arena, ctx->normalizer, ctx->typeFunctionRuntime,
  //     ctx->scope, ctx->ice, ctx->limits, location);
  let subtyping = Subtyping::subtyping_owned(
    Handle::from_nonnull(ctx.builtins),
    Handle::from_nonnull(ctx.arena),
    Some(Handle::from_nonnull(ctx.normalizer)),
    alias_ref(ctx.type_function_runtime.as_ptr()),
    alias_ref(ctx.ice.as_ptr()),
  );
  let mut resolver = OverloadResolver {
    builtin_types: Handle::from_nonnull(ctx.builtins),
    arena: Handle::from_nonnull(ctx.arena),
    normalizer: Handle::from_nonnull(ctx.normalizer),
    type_function_runtime: Handle::from_nonnull(ctx.type_function_runtime),
    scope: alias_ref(ctx.scope.as_ptr()),
    ice: Handle::from_nonnull(ctx.ice),
    // C++ `NotNull<TypeCheckLimits>` 的非拥有共享借用，与 ctx.limits 指向的
    // 会话级 TypeCheckLimits 同源；resolver 不拥有、不 drop 它。经 `Handle`
    // 门面取引用，解引用契约集中于 `arena_handle`。
    limits: Handle::from_nonnull(ctx.limits).get(),
    subtyping,
    call_loc: location,
  };

  let mut unique_types: DenseHashSet<TypeId> = DenseHashSet::default();
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

  let mut ret_pack =
    alias(ctx.arena.as_ptr()).fresh_type_pack(ctx.scope.as_ptr(), Polarity::Positive);
  let prospective_function = alias(ctx.arena.as_ptr()).add_type(FunctionType::function_type_new(
    args_pack, ret_pack, None, false,
  ));

  // FIXME (mirroring C++): we have to bust out the Unifier here.
  let mut unifier = Unifier2::unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter(
        ctx.arena,
        ctx.builtins,
        ctx.scope,
        ctx.ice,
    );

  let unify_result = unifier.unify(selected_overload, prospective_function);

  match unify_result {
    UnifyResult::Ok => {}
    UnifyResult::OccursCheckFailed => return None,
    UnifyResult::TooComplex => return None,
  }

  if !unifier.generic_substitutions.empty() || !unifier.generic_pack_substitutions.empty() {
    let mut subtyping2 = Subtyping::subtyping_owned(
      Handle::from_nonnull(ctx.builtins),
      Handle::from_nonnull(ctx.arena),
      Some(Handle::from_nonnull(ctx.normalizer)),
      alias_ref(ctx.type_function_runtime.as_ptr()),
      alias_ref(ctx.ice.as_ptr()),
    );

    let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();
    let new_ret_tp = get_approximate_return_type_for_function_call(selected_overload, &mut seen)
      .unwrap_or(alias_ref(ctx.builtins.as_ptr()).error_type_pack);

    // C++ `std::move`s the substitution maps into instantiate2; the unifier's
    // substitution fields are not read after this point (only new_fresh_types /
    // new_fresh_type_packs below), so mem::take is the faithful equivalent.
    let subst = instantiate_2_type_pack(
      Handle::from_ptr(ctx.arena.as_ptr()),
      take(&mut unifier.generic_substitutions),
      take(&mut unifier.generic_pack_substitutions),
      &mut subtyping2 as *mut Subtyping,
      alias_ref(ctx.scope.as_ptr()),
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
    track_interior_free_type(ctx.scope.as_ptr(), ty);
  }
  for &tp in &unifier.new_fresh_type_packs {
    track_interior_free_type_pack(ctx.scope.as_ptr(), tp);
  }

  Some(ret_pack)
}
