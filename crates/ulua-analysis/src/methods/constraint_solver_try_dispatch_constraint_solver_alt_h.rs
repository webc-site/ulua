use alloc::sync::Arc;
use core::ptr::{NonNull, null, null_mut};

use ulua_ast::records::{ast_node::AstNode, location::Location};
use ulua_common::{
  FFlag, FFlag as UluaCommonFFlag,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::{
  enums::{polarity::Polarity, unify_result::UnifyResult},
  functions::{
    find_unique_types_ast_utils_alt_d::find_unique_types, flatten_type_pack::flatten_type_pack_id,
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id,
    get_approximate_return_type_for_function_call_type_utils_alt_b::get_approximate_return_type_for_function_call_type_id,
    get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
    instantiate_2_instantiation_2::instantiate_2 as instantiate_2_type_id,
    instantiate_2_instantiation_2_alt_b::instantiate_2, shallow_clone_clone_alt_b::shallow_clone,
    track_interior_free_type::track_interior_free_type,
    track_interior_free_type_pack::track_interior_free_type_pack,
  },
  records::{
    any_type::AnyType, clone_state::CloneState, code_too_complex::CodeTooComplex,
    constraint::Constraint, constraint_solver::ConstraintSolver, free_type::FreeType,
    function_call_constraint::FunctionCallConstraint, function_type::FunctionType,
    generic_type_visitor::GenericTypeVisitorTrait, instantiation_queuer::InstantiationQueuer,
    instantiation_queuer_deprecated::InstantiationQueuerDeprecated,
    internal_error_reporter::InternalErrorReporter, intersection_type::IntersectionType,
    iterative_type_visitor::IterativeTypeVisitorTrait,
    magic_function_call_context::MagicFunctionCallContext,
    magic_refinement_context::MagicRefinementContext, never_type::NeverType,
    occurs_check_failed::OccursCheckFailed, overload_resolver::OverloadResolver,
    subtyping::Subtyping, unification_too_complex::UnificationTooComplex, unifier_2::Unifier2,
    union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_function_call_constraint_not_null_constraint_bool(
    &mut self,
    c: &FunctionCallConstraint,
    constraint: *const Constraint,
    _force: bool,
  ) -> bool {
    let mut fn_ty = follow_type_id(c.fn_type);
    let args_pack = unsafe { follow_type_pack_id(c.args_pack) };
    let result = unsafe { follow_type_pack_id(c.result) };

    if self.is_blocked_type_id(fn_ty) {
      return self.block_type_id_not_null_constraint(c.fn_type, constraint);
    }

    if get_type_id::<AnyType>(fn_ty).is_some() {
      unsafe {
        self.bind_not_null_constraint_type_pack_id_type_pack_id(
          constraint,
          c.result,
          (*self.builtin_types).any_type_pack,
        )
      };
      unsafe { self.fill_in_discriminant_types(constraint, &c.discriminant_types) };
      return true;
    }

    if get_type_id::<ErrorType>(fn_ty).is_some() {
      unsafe {
        self.bind_not_null_constraint_type_pack_id_type_pack_id(
          constraint,
          c.result,
          (*self.builtin_types).error_type_pack,
        )
      };
      unsafe { self.fill_in_discriminant_types(constraint, &c.discriminant_types) };
      return true;
    }

    if get_type_id::<NeverType>(fn_ty).is_some() {
      unsafe {
        self.bind_not_null_constraint_type_pack_id_type_pack_id(
          constraint,
          c.result,
          (*self.builtin_types).never_type_pack,
        )
      };
      unsafe { self.fill_in_discriminant_types(constraint, &c.discriminant_types) };
      return true;
    }

    let (args_head, args_tail) = flatten_type_pack_id(args_pack);
    let mut blocked = false;

    for arg in args_head {
      if self.is_blocked_type_id(arg) {
        self.block_type_id_not_null_constraint(arg, constraint);
        blocked = true;
      }
    }

    if let Some(tail) = args_tail
      && self.is_blocked_type_pack_id(tail)
    {
      self.block_type_pack_id_not_null_constraint(tail, constraint);
      blocked = true;
    }

    if blocked {
      return false;
    }

    let scope = unsafe { (*constraint).scope };
    let location = unsafe { (*constraint).location };

    let mut args_pack = args_pack;

    fn collapse(parts: &[TypeId]) -> Option<TypeId> {
      let first = parts.first().copied()?;
      let first = follow_type_id(first);

      for part in parts {
        if follow_type_id(*part) != first {
          return None;
        }
      }

      Some(first)
    }

    if let Some(utv) = get_type_id::<UnionType>(fn_ty) {
      fn_ty = collapse(&utv.options).unwrap_or(fn_ty);
    } else if let Some(itv) = get_type_id::<IntersectionType>(fn_ty) {
      fn_ty = collapse(&itv.parts).unwrap_or(fn_ty);
    }

    let mut used_magic = false;
    if let Some(ftv) = get_type_id::<FunctionType>(fn_ty)
      && let Some(magic) = &ftv.magic
      && !c.call_site.is_null()
    {
      used_magic = (magic.infer)(&MagicFunctionCallContext {
        solver: NonNull::new(self as *mut ConstraintSolver).unwrap(),
        constraint: NonNull::new(constraint as *mut Constraint).unwrap(),
        call_site: NonNull::new(c.call_site).unwrap(),
        arguments: c.args_pack,
        result,
      });
      (magic.refine)(&MagicRefinementContext {
        scope,
        call_site: c.call_site,
        discriminant_types: c.discriminant_types.clone(),
      });
    }

    if UluaCommonFFlag::LuauExplicitTypeInstantiationSupport.get()
      && (!c.type_arguments.is_empty() || !c.type_pack_arguments.is_empty())
    {
      fn_ty = self.instantiate_function_type(
        c.fn_type,
        &c.type_arguments,
        &c.type_pack_arguments,
        scope,
        &location,
      );
    }

    unsafe { self.fill_in_discriminant_types(constraint, &c.discriminant_types) };

    let mut overload_to_use = fn_ty;

    if get_type_id::<FunctionType>(overload_to_use).is_none() {
      let mut resolver = unsafe {
        OverloadResolver::new(
          self.builtin_types,
          self.arena,
          self.normalizer,
          self.type_function_runtime,
          scope,
          &mut self.ice_reporter as *mut InternalErrorReporter,
          &mut self.limits as *mut _,
          location,
        )
      };

      let mut unique_types: DenseHashSet<TypeId> = DenseHashSet::new(null_mut());
      if !c.call_site.is_null()
        && let Some(module) = &self.module
      {
        let module_ptr = Arc::as_ptr(module);
        unsafe {
          find_unique_types(
            &mut unique_types as *mut DenseHashSet<TypeId>,
            (*c.call_site).args.as_slice(),
            &(*module_ptr).ast_types as *const _,
          );
        }
      }

      let resolution = resolver.resolve_overload(
        overload_to_use,
        args_pack,
        if c.call_site.is_null() {
          Location::default()
        } else {
          unsafe { (*(*c.call_site).func).base.location }
        },
        &mut unique_types as *mut DenseHashSet<TypeId>,
        true,
      );

      let selected = resolution.get_unambiguous_overload();
      if let Some(overload) = selected.overload {
        overload_to_use = overload;
      } else {
        unsafe {
          self.bind_not_null_constraint_type_pack_id_type_pack_id(
            constraint,
            c.result,
            (*self.builtin_types).error_type_pack,
          )
        };
        return true;
      }

      if resolution.metamethods.contains(&overload_to_use) {
        args_pack = unsafe {
          (*self.arena)
            .add_type_pack_vector_type_id_optional_type_pack_id(alloc::vec![fn_ty], Some(args_pack))
        };
      }
    }

    let ret_tp = unsafe { (*self.arena).fresh_type_pack(scope, Polarity::Positive) };
    track_interior_free_type_pack(scope, ret_tp);

    let inferred_ty = unsafe {
      (*self.arena).add_type(FunctionType::function_type_new(
        args_pack, ret_tp, None, false,
      ))
    };

    let mut u2 = Unifier2::unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter(
            NonNull::new(self.arena).unwrap(),
            NonNull::new(self.builtin_types).unwrap(),
            NonNull::new(scope).unwrap(),
            NonNull::new(&self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter).unwrap(),
        );

    let unify_result = u2.unify(overload_to_use, inferred_ty);

    for free_ty in u2.new_fresh_types.iter().copied() {
      track_interior_free_type(scope, free_ty);
    }
    for free_tp in u2.new_fresh_type_packs.iter().copied() {
      track_interior_free_type_pack(scope, free_tp);
    }

    let mut result_tp = ret_tp;
    if !u2.generic_substitutions.empty() || !u2.generic_pack_substitutions.empty() {
      let mut subtyping = Subtyping::subtyping_owned(
        self.builtin_types,
        self.arena,
        self.normalizer,
        self.type_function_runtime,
        &self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter,
      );

      let mut has_bound = false;
      for (_, ty) in u2.generic_substitutions.iter() {
        if let Some(ftv) = get_type_id::<FreeType>(*ty) {
          let lower_bound = follow_type_id(ftv.lower_bound);
          let upper_bound = follow_type_id(ftv.upper_bound);
          has_bound = get_type_id::<NeverType>(lower_bound).is_none()
            || get_type_id::<UnknownType>(upper_bound).is_none();

          if has_bound {
            break;
          }
        }
      }

      if get_type_id::<FunctionType>(overload_to_use).is_some() && has_bound {
        let mut clone_state = CloneState {
          builtin_types: self.builtin_types,
          seen_types: DenseHashMap::new(null()),
          seen_type_packs: DenseHashMap::new(null()),
        };

        let cloned_ty =
          unsafe { shallow_clone(overload_to_use, &mut *self.arena, &mut clone_state, true) };
        // C++ `LUAU_ASSERT(clonedFn)`：clone 保持 FunctionType 变体。
        let cloned_fn =
          get_mutable_type_id::<FunctionType>(cloned_ty).expect("clone keeps FunctionType");
        cloned_fn.generics.clear();
        cloned_fn.generic_packs.clear();

        if let Some(subst) = instantiate_2_type_id(
          self.arena,
          u2.generic_substitutions.clone(),
          u2.generic_pack_substitutions.clone(),
          &mut subtyping as *mut Subtyping,
          scope,
          cloned_ty,
        ) {
          overload_to_use = follow_type_id(subst);

          if let Some(instantiated_fn) = get_type_id::<FunctionType>(overload_to_use) {
            result_tp = unsafe { follow_type_pack_id(instantiated_fn.ret_types) };
          } else {
            self.report_error_type_error_data_location(CodeTooComplex::default().into(), &location);
            result_tp = unsafe { (*self.builtin_types).error_type_pack };
          }
        } else {
          self.report_error_type_error_data_location(CodeTooComplex::default().into(), &location);
          result_tp = unsafe { (*self.builtin_types).error_type_pack };
        }
      } else {
        let approximate_ret =
          get_approximate_return_type_for_function_call_type_id(overload_to_use)
            .unwrap_or(unsafe { (*self.builtin_types).error_type_pack });

        if let Some(subst) = instantiate_2(
          self.arena,
          u2.generic_substitutions.clone(),
          u2.generic_pack_substitutions.clone(),
          &mut subtyping as *mut Subtyping,
          scope,
          approximate_ret,
        ) {
          result_tp = subst;
        } else {
          self.report_error_type_error_data_location(CodeTooComplex::default().into(), &location);
          result_tp = unsafe { (*self.builtin_types).error_type_pack };
        }
      }
    }

    if !used_magic {
      unsafe {
        self.bind_not_null_constraint_type_pack_id_type_pack_id(constraint, c.result, result_tp)
      };
    }

    for (expanded, additions) in u2.expanded_free_types.iter() {
      for addition in additions {
        self
          .upper_bound_contributors
          .get_or_insert(*expanded)
          .push((location, *addition));
      }
    }

    match unify_result {
      UnifyResult::Ok => {
        if !c.call_site.is_null() && !c.ast_overload_resolved_types.is_null() {
          unsafe {
            *(*c.ast_overload_resolved_types).get_or_insert(c.call_site as *const AstNode) =
              if used_magic {
                inferred_ty
              } else {
                overload_to_use
              };
          }
        }
      }
      UnifyResult::TooComplex => self
        .report_error_type_error_data_location(UnificationTooComplex::default().into(), &location),
      UnifyResult::OccursCheckFailed => {
        self.report_error_type_error_data_location(OccursCheckFailed::default().into(), &location)
      }
    }

    if FFlag::LuauIterativeInstantiationQueuer.get() {
      let mut queuer = InstantiationQueuer::new(
        NonNull::new(scope).unwrap(),
        &location,
        self as *mut ConstraintSolver,
      );
      queuer.run_type_id(overload_to_use);
      if FFlag::LuauAlsoInstantiateInferredArguments.get() {
        queuer.run_type_pack_id(args_pack);
      }
      queuer.run_type_pack_id(result);
    } else {
      let mut queuer = InstantiationQueuerDeprecated::instantiation_queuer_deprecated_instantiation_queuer_deprecated(
                NonNull::new(scope).unwrap(),
                &location,
                self as *mut ConstraintSolver,
            );
      queuer.traverse_type_id(overload_to_use);
      if FFlag::LuauAlsoInstantiateInferredArguments.get() {
        queuer.traverse_type_pack_id(args_pack);
      }
      queuer.traverse_type_pack_id(result);
    }

    if !FFlag::LuauConstraintGraph.get() {
      self.unblock_type_pack_id_location(c.result, location);
    }

    true
  }
}
