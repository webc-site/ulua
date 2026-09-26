//! C++ `static std::optional<TypePackId> solveFunctionCall(NotNull<TypeFunctionContext>
//! ctx, const Location& location, TypeId fnTy, TypePackId argsPack)`
//! (BuiltinTypeFunctions.cpp:121-192). Resolves a (meta)method overload, unifies a
//! prospective function shape against it, and returns the resulting return pack
//! (instantiating generic substitutions where the overload was generic).

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
    arena_handle::Handle, function_type::FunctionType, overload_resolution::OverloadResolution,
    overload_resolver::OverloadResolver, subtyping::Subtyping,
    type_function_context::TypeFunctionContext, unifier_2::Unifier2,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证 `ctx` 指向的 `TypeFunctionContext` 及其全部成员
/// （builtins/arena/normalizer/typeFunctionRuntime/scope/ice/limits）在本次调用期间存活且不被并发改写。
/// 对应 C++ `solveFunctionCall(TypeFunctionContext&, ...)` 的引用契约。
pub unsafe fn solve_function_call(
  ctx: &TypeFunctionContext,
  location: Location,
  fn_ty: TypeId,
  args_pack: TypePackId,
) -> Option<TypePackId> {
  // Safety: 入口 `unsafe fn` 契约已保证 ctx 指向对象存活于整个调用期；本次只读借用不逃逸。
  let ctx_ref = ctx;

  // auto resolver = std::make_unique<OverloadResolver>(
  //     ctx->builtins, ctx->arena, ctx->normalizer, ctx->typeFunctionRuntime,
  //     ctx->scope, ctx->ice, ctx->limits, location);
  let subtyping = Subtyping::subtyping_owned(
    Handle::from_nonnull(ctx_ref.builtins),
    Handle::from_nonnull(ctx_ref.arena),
    ctx_ref.normalizer.as_ptr(),
    ctx_ref.type_function_runtime.as_ptr(),
    ctx_ref.ice.as_ptr(),
  );
  let mut resolver = OverloadResolver {
    builtin_types: Handle::from_ptr(ctx_ref.builtins.as_ptr()),
    arena: Handle::from_ptr(ctx_ref.arena.as_ptr()),
    normalizer: Handle::from_ptr(ctx_ref.normalizer.as_ptr()),
    type_function_runtime: Handle::from_ptr(ctx_ref.type_function_runtime.as_ptr()),
    scope: ctx_ref.scope.as_ptr(),
    ice: Handle::from_ptr(ctx_ref.ice.as_ptr()),
    // C++ `NotNull<TypeCheckLimits>` 的非拥有共享借用，与 ctx.limits 指向的
    // 会话级 TypeCheckLimits 同源；resolver 不拥有、不 drop 它。
    // Safety: ctx_ref.limits 由 TypeFunctionContext 构造期布线为非空，指向的会话级对象
    // 存活期覆盖由 ctx_ref 借出的整个 resolver 生命周期；只借不降。
    limits: unsafe { ctx_ref.limits.as_ref() },
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

  // Safety: arena/scope 由 TypeFunctionContext 构造期布线为非空（C++ NotNull），
  // 指向的 arena 存活于整个类型检查会话；单线程串行独占使用。
  let mut ret_pack = unsafe {
    (*ctx_ref.arena.as_ptr()).fresh_type_pack(ctx_ref.scope.as_ptr(), Polarity::Positive)
  };
  // Safety: 同上——arena 非空且存活；add_type 只在 arena 尾部追加，不移动已分配节点。
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
      Handle::from_nonnull(ctx_ref.builtins),
      Handle::from_nonnull(ctx_ref.arena),
      ctx_ref.normalizer.as_ptr(),
      ctx_ref.type_function_runtime.as_ptr(),
      ctx_ref.ice.as_ptr(),
    );

    let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();
    let new_ret_tp = get_approximate_return_type_for_function_call(selected_overload, &mut seen)
      .unwrap_or(unsafe {
        // Safety: builtins 由 TypeFunctionContext 构造期布线为非空（C++ NotNull<BuiltinTypes>），
        // 指向对象存活于整个会话；此处只拷出一个 TypePackId 标量，引用不逃逸。
        ctx_ref.builtins.as_ref().error_type_pack
      });

    // C++ `std::move`s the substitution maps into instantiate2; a clone is the
    // faithful behavioral equivalent (the unifier is dropped at scope end).
    let subst = instantiate_2_type_pack(
      Handle::from_ptr(ctx_ref.arena.as_ptr()),
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
