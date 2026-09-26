use alloc::{sync::Arc, vec::Vec};
use core::{mem::take, ptr::NonNull};
use std::ptr::eq;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_name::AstName, ast_node::AstNode, location::Location,
};
use ulua_common::{
  fflag::{self, LuauExplicitTypeInstantiationSupport},
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::{
  blocked_early_exit,
  enums::{
    polarity::Polarity, table_state::TableState, unify_result::UnifyResult,
    value_context::ValueContext,
  },
  functions::{
    add_union::add_union,
    as_mutable_type::as_mutable_type_id,
    as_mutable_type_pack::as_mutable_type_pack,
    begin_type::{begin_intersection_type, begin_union_type},
    begin_type_pack::begin,
    can_mutate_constraint_solver::can_mutate,
    clone_clone::type_is_foreign_or_persistent,
    end_type_pack::end_type_pack_id,
    extend_type_pack::extend_type_pack,
    find_blocked_arg_types_in::find_blocked_arg_types_in,
    find_unique_types_ast_utils::find_unique_types_exprs as find_unique_types,
    flatten_type_pack::flatten_type_pack_id,
    follow_type, follow_type_pack,
    fresh_type::fresh_type,
    generalize::generalize,
    generalize_type::generalize_type,
    generalize_type_pack::generalize_type_pack,
    get_approximate_return_type_for_function_call_type_utils::get_approximate_return_type_for_function_call_type_id,
    get_constraint::get_constraint,
    get_error::get_type_error,
    get_mutable_table_type::get_mutable_table_type,
    get_mutable_type,
    get_table_type::get_table_type,
    get_type, get_type_pack,
    instantiate::instantiate,
    instantiate_2_instantiation_2::{
      instantiate_2 as instantiate_2_type_id, instantiate_2_type_pack,
    },
    is_known::is_known,
    lookup_extern_type_prop::lookup_extern_type_prop,
    maybe_singleton::maybe_singleton,
    maybe_string::maybe_string,
    occurs_check_type_utils::occurs_check_type_id_type_id,
    prune_unnecessary_generics::prune_unnecessary_generics,
    push_type_into::push_type_into,
    reduce_type_functions_type_function::{
      reduce_type_functions, reduce_type_functions_type_pack_id as reduce_type_functions_tp,
    },
    saturate_arguments::saturate_arguments,
    seal_table::seal_table,
    shallow_clone_clone::shallow_clone,
    track_interior_free_type::track_interior_free_type,
    track_interior_free_type_pack::track_interior_free_type_pack,
    unwrap_group::unwrap_group,
  },
  methods::unifiable_bound_type_id_emplace_type_bound_type::unifiable_bound_type_id_emplace_type_bound_type,
  records::{
    any_type::AnyType,
    apply_type_function::ApplyTypeFunction,
    arena_handle::Handle,
    assign_index_constraint::AssignIndexConstraint,
    assign_prop_constraint::AssignPropConstraint,
    blocked_constraint_registry::register_constraint,
    blocked_type::BlockedType,
    blocked_type_finder::BlockedTypeFinder,
    clone_state::CloneState,
    code_too_complex::CodeTooComplex,
    constraint::Constraint,
    constraint_solver::ConstraintSolver,
    equality_constraint::EqualityConstraint,
    extern_type::ExternType,
    find_all_union_members::FindAllUnionMembers,
    free_type::FreeType,
    free_type_pack::FreeTypePack,
    function_call_constraint::FunctionCallConstraint,
    function_check_constraint::FunctionCheckConstraint,
    function_type::FunctionType,
    generalization_constraint::GeneralizationConstraint,
    generalization_params::GeneralizationParams,
    generic_type::GenericType,
    generic_type_visitor::GenericTypeVisitorTrait,
    has_indexer_constraint::HasIndexerConstraint,
    has_prop_constraint::HasPropConstraint,
    infinite_type_finder::InfiniteTypeFinder,
    instantiation_queuer::InstantiationQueuer,
    instantiation_queuer_deprecated::InstantiationQueuerDeprecated,
    instantiation_signature::InstantiationSignature,
    internal_error_reporter::InternalErrorReporter,
    intersection_type::IntersectionType,
    iterable_constraint::IterableConstraint,
    iterative_type_visitor::IterativeTypeVisitorTrait,
    magic_function_call_context::MagicFunctionCallContext,
    magic_refinement_context::MagicRefinementContext,
    metatable_type::MetatableType,
    name_constraint::NameConstraint,
    never_type::NeverType,
    occurs_check_failed::OccursCheckFailed,
    overload_resolver::OverloadResolver,
    pack_subtype_constraint::PackSubtypeConstraint,
    pending_expansion_type::PendingExpansionType,
    primitive_type_constraint::PrimitiveTypeConstraint,
    property_type::Property,
    push_function_type_constraint::PushFunctionTypeConstraint,
    push_type_constraint::PushTypeConstraint,
    reduce_constraint::ReduceConstraint,
    reduce_pack_constraint::ReducePackConstraint,
    simplify_constraint::SimplifyConstraint,
    subtype_constraint::SubtypeConstraint,
    subtyping::Subtyping,
    table_indexer::TableIndexer,
    table_type::TableType,
    type_alias_expansion_constraint::TypeAliasExpansionConstraint,
    type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
    type_ids::TypeIds,
    type_instantiation_constraint::TypeInstantiationConstraint,
    type_level::TypeLevel,
    unification_too_complex::UnificationTooComplex,
    unifier_2::Unifier2,
    uninhabited_type_function::UninhabitedTypeFunction,
    uninhabited_type_pack_function::UninhabitedTypePackFunction,
    union_type::UnionType,
    unknown_symbol::{Context, UnknownSymbol},
    unknown_type::UnknownType,
    unpack_constraint::UnpackConstraint,
  },
  type_aliases::{
    blocked_constraint_id::BlockedConstraintId, constraint_v::ConstraintV, error_type::ErrorType,
    props_type::Props, type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant, type_variant::TypeVariant,
  },
};

/// `NonNull::new(..).expect(..)` 的非空契约文案单点表：原 40 余处逐字重复的
/// expect 字符串收敛为常量（panic 文案不变，语义不变）。
mod nonnull_contract {
  /// cpp `NotNull<Scope>` 契约：`Constraint.scope` 构造期接线，恒非空。
  pub(crate) const CONSTRAINT_SCOPE: &str =
    "Constraint.scope 按 cpp NotNull<Scope> 契约构造期接线，恒非空";
  /// `scope` 值即 `constraint.scope`（cpp NotNull 契约），恒非空。
  pub(crate) const SCOPE_IS_CONSTRAINT_SCOPE: &str =
    "scope 即 constraint.scope（cpp NotNull<Scope> 契约），恒非空";
  /// `Handle<T>` 内持 `NonNull`，`as_ptr` 恒非空。
  pub(crate) const HANDLE_AS_PTR: &str = "Handle 内持 NonNull，as_ptr 恒非空";
  /// 正在派发的约束借自 `&Constraint`，转裸指针恒非空。
  pub(crate) const CONSTRAINT_REF: &str = "正在派发的约束借自 &Constraint，转裸指针恒非空";
  /// `&mut self` 转裸指针恒非空。
  pub(crate) const SELF_AS_PTR: &str = "&mut self 转裸指针恒非空";
  /// 局部 `&mut` 转裸指针恒非空。
  pub(crate) const LOCAL_MUT_AS_PTR: &str = "局部 &mut 转裸指针恒非空";
  /// subtyping 由构造期 `Handle(NonNull)` 接线为 `*mut`，恒非空。
  pub(crate) const SUBTYPING: &str = "subtyping 由构造期 Handle(NonNull) 接线为 *mut，恒非空";
  /// `&` 借用转裸指针恒非空。
  pub(crate) const REF_AS_PTR: &str = "& 借用转裸指针恒非空";
}
use nonnull_contract as nc;

impl ConstraintSolver {
  /// C++ `tryDispatch(NotNull<const Constraint> c, bool force)`：`constraint`
  /// 来自 `solver_constraints`（`Vec<Box<Constraint>>`），非空由构造保证。
  pub fn try_dispatch_not_null_constraint_bool(
    &mut self,
    constraint: &Constraint,
    force: bool,
  ) -> bool {
    if fflag::LuauConstraintGraph.get() {
      LUAU_ASSERT!(
        force
          || !self
            .cgraph_mut()
            .has_unsolved_dependencies(BlockedConstraintId::V2(register_constraint(
              constraint as *const Constraint,
            )))
      );
    } else {
      if !force && self.deprecate_d_is_blocked(constraint as *const Constraint) {
        return false;
      }
    }

    let mut success = false;
    let c = constraint;

    if let Some(sc) = get_constraint::<SubtypeConstraint>(c) {
      success = self.try_dispatch_subtype_constraint_not_null_constraint(sc, constraint);
    } else if let Some(psc) = get_constraint::<PackSubtypeConstraint>(c) {
      success = self.try_dispatch_pack_subtype_constraint_not_null_constraint(psc, constraint);
    } else if let Some(gc) = get_constraint::<GeneralizationConstraint>(c) {
      success = self.try_dispatch_generalization_constraint_not_null_constraint(gc, constraint);
    } else if let Some(ic) = get_constraint::<IterableConstraint>(c) {
      success =
        self.try_dispatch_iterable_constraint_not_null_constraint_bool(ic, constraint, force);
    } else if let Some(nc) = get_constraint::<NameConstraint>(c) {
      success = self.try_dispatch_name_constraint_not_null_constraint(nc, constraint);
    } else if let Some(taec) = get_constraint::<TypeAliasExpansionConstraint>(c) {
      success =
        self.try_dispatch_type_alias_expansion_constraint_not_null_constraint(taec, constraint);
    } else if let Some(fcc) = get_constraint::<FunctionCallConstraint>(c) {
      success =
        self.try_dispatch_function_call_constraint_not_null_constraint_bool(fcc, constraint, force);
    } else if let Some(fcc) = get_constraint::<FunctionCheckConstraint>(c) {
      success = self
        .try_dispatch_function_check_constraint_not_null_constraint_bool(fcc, constraint, force);
    } else if let Some(pc) = get_constraint::<PrimitiveTypeConstraint>(c) {
      success = self.try_dispatch_primitive_type_constraint_not_null_constraint(pc, constraint);
    } else if let Some(hpc) = get_constraint::<HasPropConstraint>(c) {
      success = self.try_dispatch_has_prop_constraint_not_null_constraint(hpc, constraint);
    } else if let Some(spc) = get_constraint::<HasIndexerConstraint>(c) {
      success = self.try_dispatch_has_indexer_constraint_not_null_constraint(spc, constraint);
    } else if let Some(uc) = get_constraint::<AssignPropConstraint>(c) {
      success = self.try_dispatch_assign_prop_constraint_not_null_constraint(uc, constraint);
    } else if let Some(uc) = get_constraint::<AssignIndexConstraint>(c) {
      success = self.try_dispatch_assign_index_constraint_not_null_constraint(uc, constraint);
    } else if let Some(uc) = get_constraint::<UnpackConstraint>(c) {
      success = self.try_dispatch_unpack_constraint_not_null_constraint(uc, constraint);
    } else if let Some(rc) = get_constraint::<ReduceConstraint>(c) {
      success = self.try_dispatch_reduce_constraint_not_null_constraint_bool(rc, constraint, force);
    } else if let Some(rpc) = get_constraint::<ReducePackConstraint>(c) {
      success =
        self.try_dispatch_reduce_pack_constraint_not_null_constraint_bool(rpc, constraint, force);
    } else if let Some(eqc) = get_constraint::<EqualityConstraint>(c) {
      success = self.try_dispatch_equality_constraint_not_null_constraint(eqc, constraint);
    } else if let Some(sc) = get_constraint::<SimplifyConstraint>(c) {
      success =
        self.try_dispatch_simplify_constraint_not_null_constraint_bool(sc, constraint, force);
    } else if let Some(pftc) = get_constraint::<PushFunctionTypeConstraint>(c) {
      success =
        self.try_dispatch_push_function_type_constraint_not_null_constraint(pftc, constraint);
    } else if let Some(esgc) = get_constraint::<TypeInstantiationConstraint>(c) {
      LUAU_ASSERT!(fflag::LuauExplicitTypeInstantiationSupport.get());
      success =
        self.try_dispatch_type_instantiation_constraint_not_null_constraint(esgc, constraint);
    } else if let Some(ptc) = get_constraint::<PushTypeConstraint>(c) {
      success =
        self.try_dispatch_push_type_constraint_not_null_constraint_bool(ptc, constraint, force);
    } else {
      LUAU_ASSERT!(false);
    }

    success
  }

  pub fn try_dispatch_subtype_constraint_not_null_constraint(
    &mut self,
    c: &SubtypeConstraint,
    constraint: &Constraint,
  ) -> bool {
    blocked_early_exit!(self, type c.sub_type, constraint);
    blocked_early_exit!(self, type c.super_type, constraint);

    self.constraint_solver_unify(constraint, c.sub_type, c.super_type);

    true
  }

  pub fn try_dispatch_pack_subtype_constraint_not_null_constraint(
    &mut self,
    c: &PackSubtypeConstraint,
    constraint: &Constraint,
  ) -> bool {
    blocked_early_exit!(self, pack c.sub_pack, constraint);
    blocked_early_exit!(self, pack c.super_pack, constraint);

    self.constraint_solver_unify(constraint, c.sub_pack, c.super_pack);

    true
  }

  pub fn try_dispatch_generalization_constraint_not_null_constraint(
    &mut self,
    c: &GeneralizationConstraint,
    constraint: &Constraint,
  ) -> bool {
    let generalized_type = follow_type::follow(c.generalized_type);

    blocked_early_exit!(self, type c.source_type, constraint);
    if get_type::get::<PendingExpansionType>(generalized_type).is_some() {
      return self.block_type_id_not_null_constraint(generalized_type, constraint);
    }

    let generalized_ty = generalize(
      self.arena,
      self.builtin_types,
      constraint.scope,
      &mut self.generalized_types_ as *mut _,
      c.source_type,
      None,
    );

    if generalized_ty.is_none() {
      self.report_error_type_error_data_location(
        CodeTooComplex::default().into(),
        &constraint.location,
      );
    }

    if let Some(generalized_ty) = generalized_ty {
      // Safety: `prune_unnecessary_generics` 为 unsafe fn，其契约要求四个裸指针
      // 参数有效：`arena`/`builtin_types` 由构造方以 NotNull 语义写入且在 `&mut self`
      // 期间存活；`constraint.scope` 是 C++ `NotNull<Scope>`（Constraint.h 构造即非空）；
      // `generalized_types_`/`generalized_ty` 均来自上方 `generalize` 对同一 solver 的操作。
      unsafe {
        prune_unnecessary_generics(
          self.arena,
          self.builtin_types,
          constraint.scope,
          &mut self.generalized_types_ as *mut _,
          generalized_ty,
        )
      };

      if get_type::get::<BlockedType>(generalized_type).is_some() {
        // `generalized_type` 已证为 BlockedType，可改性判定与 cpp
        // tryDispatch(GeneralizationConstraint) 的 bind 分支同一形状。
        self.bind(constraint, generalized_type, generalized_ty);
      } else {
        self.constraint_solver_unify(constraint, generalized_type, generalized_ty);
      }

      if c.has_deprecated_attribute
        && let Some(fty) =
          get_mutable_type::get_mutable::<FunctionType>(follow_type::follow(generalized_type))
      {
        fty.is_deprecated_function = true;
        fty.deprecated_info = Some(Arc::new(c.deprecated_info.clone()));
      }
    } else {
      self.report_error_type_error_data_location(
        CodeTooComplex::default().into(),
        &constraint.location,
      );
      // 本分支把 `c.generalized_type` 绑到错误类型兜底，与 cpp 失败路径
      // `bind(constraint, c.generalizedType, builtinTypes->errorType)` 逐字对应。
      let error_type = self.builtin_types_ref().error_type;
      self.bind(constraint, c.generalized_type, error_type);
    }

    let scope = constraint.scope_ref();

    if let Some(interior_free_types) = scope.interior_free_types.as_ref() {
      let interior_free_types = interior_free_types.clone();
      for ty in interior_free_types {
        let ty = follow_type::follow(ty);
        let free_ty = get_type::get::<FreeType>(ty);

        if let Some(free_ty) = free_ty {
          let params = GeneralizationParams {
            found_outside_functions: true,
            use_count: 1,
            polarity: free_ty.polarity,
          };
          // Safety: `generalize_type` 为 unsafe fn，要求 arena/builtin_types/scope
          // 指针存活——`ty` 是刚 follow 并确认为 FreeType 的 arena 节点，
          // `params` 借用其 polarity 构造于本地，scope 为约束内 NotNull<Scope>。
          let res = unsafe {
            generalize_type(
              self.arena,
              self.builtin_types,
              constraint.scope,
              ty,
              &params,
            )
          };
          if res.resource_limits_exceeded {
            self.report_error_type_error_data_location(
              CodeTooComplex::default().into(),
              &scope.location,
            );
          }
        } else if get_type::get::<TableType>(ty).is_some() {
          seal_table(constraint.scope, ty);
        }

        self.unblock_type_id_location(ty, constraint.location);
      }
    }

    if let Some(interior_free_type_packs) = scope.interior_free_type_packs.as_ref() {
      let interior_free_type_packs = interior_free_type_packs.clone();
      for tp in interior_free_type_packs {
        let tp = follow_type_pack::follow(tp);
        let free_tp = get_type_pack::get::<FreeTypePack>(tp);

        if let Some(free_tp) = free_tp {
          let params = GeneralizationParams {
            found_outside_functions: true,
            use_count: 1,
            polarity: free_tp.polarity,
          };
          LUAU_ASSERT!(is_known(params.polarity));
          // Safety: `generalize_type_pack` 的 unsafe 契约同上——`tp` 已 follow 且
          // 确认为 FreeTypePack（上方 `get_type_pack_id` 命中才进入），
          // arena/builtin_types/scope 均为 solver 构造期 NotNull 指针。
          unsafe {
            generalize_type_pack(
              self.arena,
              self.builtin_types,
              constraint.scope,
              tp,
              &params,
            )
          };
        }
      }
    }

    if c.no_generics
      && let Some(ft) = get_mutable_type::get_mutable::<FunctionType>(c.source_type)
    {
      // Safety: `ft` 来自 `get_mutable_type_id`，即 arena 内存活类型节点的独占
      // 视图（C++ `getMutable<FunctionType>` 同契约）；`as_mutable_type_id` 只是
      // 对其中 Generic TypeId（arena 分配、永不为空）做 const→mut 转换，改写
      // `.ty` 不影响借用图；内联解引用 `builtin_types` 由构造契约保证有效。
      unsafe {
        for r#gen in ft.generics.iter().copied() {
          (*as_mutable_type_id(r#gen)).ty =
            TypeVariant::Bound(self.builtin_types.get().unknown_type);
        }
        ft.generics.clear();

        for r#gen in ft.generic_packs.iter().copied() {
          (*as_mutable_type_pack(r#gen)).ty =
            TypePackVariant::Bound(self.builtin_types_ref().unknown_type_pack);
        }
        ft.generic_packs.clear();
      }
    }

    true
  }

  pub fn try_dispatch_iterable_constraint_not_null_constraint_bool(
    &mut self,
    c: &IterableConstraint,
    constraint: &Constraint,
    mut force: bool,
  ) -> bool {
    if fflag::LuauForceLess.get() {
      // cpp: `if (FFlag::LuauForceLess) force = false;`
      force = false;
    }

    // SAFETY: arena/builtin_types 在 solver 存续期内有效（NotNull 契约）。
    let iterator = unsafe {
      extend_type_pack(
        self.arena.get_mut(),
        Handle::from_ptr(self.builtin_types.as_ptr()),
        c.iterator,
        3,
        Vec::new(),
      )
    };

    if iterator.head.len() < 3
      && let Some(tail) = iterator.tail
      && self.is_blocked_type_pack_id(tail)
    {
      return if force {
        true
      } else {
        self.block_type_pack_id_not_null_constraint(tail, constraint)
      };
    }

    let mut blocked = false;
    for ty in &iterator.head {
      if self.is_blocked_type_id(*ty) {
        self.block_type_id_not_null_constraint(*ty, constraint);
        blocked = true;
      }
    }

    if blocked {
      return false;
    }

    if iterator.head.is_empty() {
      for ty in &c.variables {
        // 迭代器空包 → 变量全部绑 error（cpp 同分支）。
        let error_type = self.builtin_types_ref().error_type;
        self.bind(constraint, *ty, error_type);
      }
      return true;
    }

    let next_ty = follow_type::follow(iterator.head[0]);
    if get_type::get::<FreeType>(next_ty).is_some() {
      let scope = constraint.scope;
      // SAFETY: `fresh_type` 是安全函数，unsafe 仅因为拆分借用需解引用裸指针：
      // `self.arena.get_mut()` 独占 arena 分配，`self.builtin_types.get()` 只读另一个
      // 不相交字段的堆对象，两者均由构造契约保证非空且随 solver 存活。
      let key_ty = {
        fresh_type(
          self.arena.get_mut(),
          self.builtin_types.get(),
          scope,
          Polarity::Mixed,
        )
      };
      // SAFETY: 同上——arena 与 builtin_types 指向互不相交的分配，本次派生的
      // 共享/独占借用不重叠；scope 为约束内 NotNull<Scope> 指针。
      let value_ty = {
        fresh_type(
          self.arena.get_mut(),
          self.builtin_types.get(),
          scope,
          Polarity::Mixed,
        )
      };
      track_interior_free_type(scope, key_ty);
      track_interior_free_type(scope, value_ty);

      let props = Props::default();
      let table_ty = self.arena_mut().add_type(
        TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
          &props,
          Some(TableIndexer {
            index_type: key_ty,
            index_result_type: value_ty,
            is_read_only: false,
          }),
          TypeLevel::default(),
          scope,
          TableState::Free,
        ),
      );
      track_interior_free_type(scope, table_ty);

      self.constraint_solver_unify(constraint, next_ty, table_ty);

      let mut it = c.variables.iter();
      if let Some(ty) = it.next() {
        // 首个迭代变量尚未定型，bind 前置条件与 cpp tryDispatch(IterableConstraint)
        // 的 key 分支一致。
        self.bind(constraint, *ty, key_ty);
      }
      if let Some(ty) = it.next() {
        // 第二个迭代变量（value 位）绑 value_ty；与 C++ `bind(constraint, *var++, keyTy)`
        // 序列的第二步对应。
        self.bind(constraint, *ty, value_ty);
      }
      for ty in it {
        // 其余多余变量按 C++ 语义绑 nil。
        let nil_type = self.builtin_types_ref().nil_type;
        self.bind(constraint, *ty, nil_type);
      }

      return true;
    }

    if get_type::get::<FunctionType>(next_ty).is_some() {
      let table_ty = iterator
        .head
        .get(1)
        .copied()
        .unwrap_or_else(|| self.builtin_types_ref().nil_type);

      return self.try_dispatch_iterable_function(next_ty, table_ty, c, constraint);
    }

    self.try_dispatch_iterable_table(iterator.head[0], c, constraint, force)
  }

  pub fn try_dispatch_name_constraint_not_null_constraint(
    &mut self,
    c: &NameConstraint,
    constraint: &Constraint,
  ) -> bool {
    let scope = constraint.scope_ref();
    blocked_early_exit!(self, type c.named_type, constraint);

    let target = follow_type::follow(c.named_type);

    // `target` 为持久类型或归属他 arena 时不就地改名（cpp isPersistent/getOwningArena）。
    if type_is_foreign_or_persistent(target, self.arena.get().arena_id) {
      return true;
    }

    if let Some(tf) = scope.lookup_type(&c.name) {
      let signature = InstantiationSignature {
        fn_sig: tf,
        arguments: c.type_parameters.clone(),
        pack_arguments: c.type_pack_parameters.clone(),
      };

      let mut itf = InfiniteTypeFinder::infinite_type_finder_infinite_type_finder(
        self,
        &signature,
        NonNull::new(constraint.scope)
          .expect(nc::CONSTRAINT_SCOPE),
      );
      itf.run_type_id(target);

      if itf.found_infinite_type {
        // SAFETY: 经 scope 裸指针写入 invalid_type_aliases——crate 图谱惯例为
        // 共享可变（C++ `Scope*` 语义），与原实现一致。
        unsafe {
          (*constraint.scope)
            .invalid_type_aliases
            .try_insert(c.name.clone(), constraint.location)
        };
        // `target` 已 follow 且证明为本 arena 类型，绑 error 对应 cpp 无限类型兜底路径。
        let error_type = self.builtin_types_ref().error_type;
        self.bind(constraint, target, error_type);
        return true;
      }
    }

    if let Some(ttv) = get_mutable_type::get_mutable::<TableType>(target) {
      if c.synthetic && ttv.name.is_none() {
        ttv.synthetic_name = Some(c.name.clone());
      } else {
        ttv.name = Some(c.name.clone());
        ttv.instantiated_type_params = c.type_parameters.clone();
        ttv.instantiated_type_pack_params = c.type_pack_parameters.clone();
      }
    } else if let Some(mtv) = get_mutable_type::get_mutable::<MetatableType>(target) {
      mtv.synthetic_name = Some(c.name.clone());
    }

    true
  }

  pub fn try_dispatch_type_alias_expansion_constraint_not_null_constraint(
    &mut self,
    c: &TypeAliasExpansionConstraint,
    constraint: &Constraint,
  ) -> bool {
    let Some(petv) = get_type::get::<PendingExpansionType>(follow_type::follow(c.target)) else {
      self.unblock_type_id_location(c.target, constraint.location);
      return true;
    };

    let alias_name = ast_name_to_string(petv.name);
    let alias_prefix = petv.prefix.map(ast_name_to_string);
    let raw_type_arguments = petv.type_arguments.clone();
    let raw_pack_arguments = petv.pack_arguments.clone();

    let scope = constraint.scope_ref();
    let tf = if let Some(prefix) = &alias_prefix {
      scope.lookup_imported_type(prefix, &alias_name)
    } else {
      scope.lookup_type(&alias_name)
    };

    let Some(tf) = tf else {
      self.report_error_type_error_data_location(
        TypeErrorData::UnknownSymbol(UnknownSymbol::new(alias_name, Context::Type)),
        &constraint.location,
      );
      bind_alias_expansion_result(self, c, constraint, self.builtin_types_ref().error_type);
      return true;
    };

    if get_type::get::<TypeFunctionInstanceType>(follow_type::follow(tf.r#type())).is_some() {
      self.push_constraint(
        NonNull::new(constraint.scope)
          .expect(nc::CONSTRAINT_SCOPE),
        constraint.location,
        ConstraintV::Reduce(ReduceConstraint { ty: tf.r#type() }),
      );
    }

    let lhs = follow_type::follow(c.target);
    let rhs = tf.r#type();
    if occurs_check_type_id_type_id(lhs, rhs) {
      self.report_error_type_error_data_location(
        TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
        &constraint.location,
      );
      bind_alias_expansion_result(self, c, constraint, self.builtin_types_ref().error_type);
      return true;
    }

    if tf.type_params().is_empty() && tf.type_pack_params().is_empty() {
      bind_alias_expansion_result(self, c, constraint, tf.r#type());
      return true;
    }

    // Safety: `saturate_arguments` 是安全函数，unsafe 只来自拆分借用：
    // `self.arena.get_mut()` 与 `self.builtin_types.get_mut()` 是 `self` 上两个互不相交
    // 字段的裸指针重借用（同 C++ `*arena`/`builtinTypes` 双 NonNull 入参），
    // 均非空且随 solver 存活；`tf`/参数向量借用本地 clone 数据。
    let (type_arguments, pack_arguments) = {
      saturate_arguments(
        self.arena.get_mut(),
        self.builtin_types.get_mut(),
        &tf,
        &raw_type_arguments,
        &raw_pack_arguments,
      )
    };

    let same_types = type_arguments.len() == tf.type_params().len()
      && type_arguments
        .iter()
        .zip(tf.type_params())
        .all(|(arg, param)| *arg == param.ty);
    let same_packs = pack_arguments.len() == tf.type_pack_params().len()
      && pack_arguments
        .iter()
        .zip(tf.type_pack_params())
        .all(|(arg, param)| *arg == param.tp);

    if same_types && same_packs {
      bind_alias_expansion_result(self, c, constraint, tf.r#type());
      return true;
    }

    let signature = InstantiationSignature {
      fn_sig: tf.clone(),
      arguments: type_arguments.clone(),
      pack_arguments: pack_arguments.clone(),
    };

    if let Some(cached) = self.instantiated_aliases.find(&signature).copied() {
      bind_alias_expansion_result(self, c, constraint, cached);
      return true;
    }

    let mut itf = InfiniteTypeFinder::infinite_type_finder_infinite_type_finder(
      self,
      &signature,
      NonNull::new(constraint.scope)
        .expect(nc::CONSTRAINT_SCOPE),
    );
    itf.run_type_id(tf.r#type());

    if itf.found_infinite_type {
      bind_alias_expansion_result(self, c, constraint, self.builtin_types_ref().error_type);
      // SAFETY: 经 scope 裸指针写入 invalid_type_aliases——crate 图谱惯例为
      // 共享可变（C++ `Scope*` 语义），此处与原实现一致。
      unsafe {
        (*constraint.scope)
          .invalid_type_aliases
          .try_insert(alias_name.clone(), constraint.location);
      }
      return true;
    }

    let mut apply_type_function = ApplyTypeFunction::new(self.arena);
    for (i, ty) in type_arguments.iter().enumerate() {
      *apply_type_function
        .type_arguments
        .get_or_insert(tf.type_params()[i].ty) = *ty;
    }

    for (i, tp) in pack_arguments.iter().enumerate() {
      *apply_type_function
        .type_pack_arguments
        .get_or_insert(tf.type_pack_params()[i].tp) = *tp;
    }

    let Some(mut instantiated) = apply_type_function.substitute_type_id(tf.r#type()) else {
      bind_alias_expansion_result(self, c, constraint, self.builtin_types_ref().error_type);
      return true;
    };

    let mut target = follow_type::follow(instantiated);

    if fflag::LuauIterativeInstantiationQueuer.get() {
      let mut queuer = InstantiationQueuer::new(
        NonNull::new(constraint.scope)
          .expect(nc::CONSTRAINT_SCOPE),
        &constraint.location,
        self as *mut ConstraintSolver,
      );
      queuer.run_type_id(target);
    } else {
      let mut queuer = InstantiationQueuerDeprecated::instantiation_queuer_deprecated_instantiation_queuer_deprecated(
                NonNull::new(constraint.scope).expect(nc::CONSTRAINT_SCOPE),
                &constraint.location,
                self as *mut ConstraintSolver,
            );
      queuer.traverse_type_id(target);
    }

    // `target` 为持久或他 arena 类型时不就地改写（C++ isPersistent/getOwningArena）。
    if type_is_foreign_or_persistent(target, self.arena.get().arena_id) {
      bind_alias_expansion_result(self, c, constraint, target);
      return true;
    }

    // 指针相等对应 C++ 的 tfTable == targetTable。
    let tf_table = get_table_type(tf.r#type()).map(|table| table as *const TableType);
    let target_table = get_table_type(target).map(|table| table as *const TableType);
    let needs_clone = follow_type::follow(tf.r#type()) == target
      || (tf_table.is_some() && tf_table == target_table)
      || type_arguments.contains(&target);

    let mut table = get_mutable_table_type(target);
    if table.is_some() {
      if needs_clone {
        if get_type::get::<MetatableType>(target).is_some() {
          // SAFETY: builtin_types 在 solver 存活期内有效。
          let mut clone_state = { CloneState::new(self.builtin_types.get_mut()) };
          // SAFETY: arena 在 solver 存活期内有效。
          instantiated =
            unsafe { shallow_clone(target, self.arena.get_mut(), &mut clone_state, true) };
          let metatable = get_mutable_type::get_mutable::<MetatableType>(instantiated)
            .expect("shallow_clone 保留 MetatableType 变体");
          // Safety: `metatable.table()` 是刚 clone 出的 arena 内表类型（非空
          // TypeId）；`shallow_clone` 的 unsafe 契约要求目标类型与 arena 存活——
          // clone_state、`self.arena.get_mut()`（构造期 NotNull 字段）均满足，
          // 且除 `metatable` 外无人借用同一节点。
          metatable.table = unsafe {
            shallow_clone(
              metatable.table(),
              self.arena.get_mut(),
              &mut clone_state,
              true,
            )
          };
          table = get_mutable_table_type(metatable.table());
        } else if get_type::get::<TableType>(target).is_some() {
          // SAFETY: builtin_types 在 solver 存活期内有效。
          let mut clone_state = { CloneState::new(self.builtin_types.get_mut()) };
          // SAFETY: arena 在 solver 存活期内有效。
          instantiated =
            unsafe { shallow_clone(target, self.arena.get_mut(), &mut clone_state, true) };
          table = get_mutable_table_type(instantiated);
        }

        target = follow_type::follow(instantiated);
      }

      // C++ 对 table 直解引用；外层已确认 Some，重赋值来源同为 clone，必仍命中。
      let table =
        table.expect("外层 is_some() 守卫内 table 未被改写，且 clone 保持 TableType 变体，必命中");
      table.instantiated_type_params = type_arguments.clone();
      table.instantiated_type_pack_params = pack_arguments.clone();
      table.definition_location = constraint.location;
      if let Some(module) = &self.module {
        table.definition_module_name = module.name.clone();
      }
    }

    bind_alias_expansion_result(self, c, constraint, target);
    self.instantiated_aliases.try_insert(signature, target);

    true
  }
}

fn bind_alias_expansion_result(
  solver: &mut ConstraintSolver,
  c: &TypeAliasExpansionConstraint,
  constraint: &Constraint,
  result: TypeId,
) {
  let c_target = follow_type::follow(c.target);

  if occurs_check_type_id_type_id(c_target, result) {
    solver.report_error_type_error_data_location(
      TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
      &constraint.location,
    );
    // occurs-check 失败时按 cpp 原样把目标绑到 error 类型。
    let error_type = solver.builtin_types_ref().error_type;
    solver.bind(constraint, c_target, error_type);
  } else {
    // 成功路径绑定 `result`（follow 后的 arena 节点），对应 C++
    // `bind(constraint, cTarget, result)`。
    solver.bind(constraint, c_target, result);
  }
}

fn ast_name_to_string(name: AstName) -> String {
  name.as_str_or_empty().to_string()
}

impl ConstraintSolver {
  pub fn try_dispatch_function_call_constraint_not_null_constraint_bool(
    &mut self,
    c: &FunctionCallConstraint,
    constraint: &Constraint,
    _force: bool,
  ) -> bool {
    let mut fn_ty = follow_type::follow(c.fn_type);
    let args_pack = follow_type_pack::follow(c.args_pack);
    let result = follow_type_pack::follow(c.result);

    blocked_early_exit!(self, type fn_ty => c.fn_type, constraint);

    if get_type::get::<AnyType>(fn_ty).is_some() {
      // 函数值为 any → 结果包直接绑 any_type_pack（cpp 同分支）。
      let any_pack = self.builtin_types_ref().any_type_pack;
      self.bind_pack(constraint, c.result, any_pack);
      self.fill_in_discriminant_types(constraint, &c.discriminant_types);
      return true;
    }

    if get_type::get::<ErrorType>(fn_ty).is_some() {
      // 被调用者是 error 类型时结果包绑 error_type_pack。
      let err_pack = self.builtin_types_ref().error_type_pack;
      self.bind_pack(constraint, c.result, err_pack);
      self.fill_in_discriminant_types(constraint, &c.discriminant_types);
      return true;
    }

    if get_type::get::<NeverType>(fn_ty).is_some() {
      // never 调用永不返回，结果包绑 never_type_pack（cpp 原样）。
      let never_pack = self.builtin_types_ref().never_type_pack;
      self.bind_pack(constraint, c.result, never_pack);
      self.fill_in_discriminant_types(constraint, &c.discriminant_types);
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

    let scope = constraint.scope;
    let location = constraint.location;

    let mut args_pack = args_pack;

    // C++ `collapse` 用 `begin(t)/end(t)` 全量迭代——TypeIterator 展平
    // 嵌套同类型并 follow,裸切片会漏掉嵌套成员。
    fn collapse(parts: &[TypeId]) -> Option<TypeId> {
      let first = parts.first().copied()?;
      let first = follow_type::follow(first);

      for part in parts {
        if follow_type::follow(*part) != first {
          return None;
        }
      }

      Some(first)
    }

    if let Some(utv) = get_type::get::<UnionType>(fn_ty) {
      let flattened: Vec<TypeId> = begin_union_type(utv).collect();
      fn_ty = collapse(&flattened).unwrap_or(fn_ty);
    } else if let Some(itv) = get_type::get::<IntersectionType>(fn_ty) {
      let flattened: Vec<TypeId> = begin_intersection_type(itv).collect();
      fn_ty = collapse(&flattened).unwrap_or(fn_ty);
    }

    let mut used_magic = false;
    if let Some(ftv) = get_type::get::<FunctionType>(fn_ty)
      && let Some(magic) = &ftv.magic
      && !c.call_site.is_null()
    {
      used_magic = (magic.infer)(&MagicFunctionCallContext {
        solver: NonNull::new(self as *mut ConstraintSolver).expect(nc::SELF_AS_PTR),
        constraint: NonNull::new(constraint as *const Constraint as *mut Constraint)
          .expect(nc::CONSTRAINT_REF),
        call_site: NonNull::new(c.call_site).expect("上方 !c.call_site.is_null() 守卫已排除空指针"),
        arguments: c.args_pack,
        result,
      });
      (magic.refine)(&MagicRefinementContext {
        scope,
        call_site: c.call_site,
        discriminant_types: c.discriminant_types.clone(),
      });
    }

    if fflag::LuauExplicitTypeInstantiationSupport.get()
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

    self.fill_in_discriminant_types(constraint, &c.discriminant_types);

    let mut overload_to_use = fn_ty;

    if get_type::get::<FunctionType>(overload_to_use).is_none() {
      // Safety: `OverloadResolver::new` 是 unsafe fn，契约即 cpp 构造函数对
      // `NotNull` 参数的要求：`builtin_types`/`arena` 为 solver 构造期写入、整次
      // 求解存活的句柄（非空由 Handle 类型编码）；`normalizer`/
      // `type_function_runtime` 为同源不空指针；`scope` 来自当前约束（NotNull<Scope>），
      // reporter/limits 借用 self 字段。
      let mut resolver = unsafe {
        OverloadResolver::new(
          self.builtin_types,
          self.arena,
          self.normalizer.as_ptr(),
          self.type_function_runtime.as_ptr(),
          scope,
          &mut self.ice_reporter as *mut InternalErrorReporter,
          &mut self.limits as *mut _,
          location,
        )
      };

      let mut unique_types: DenseHashSet<TypeId> = DenseHashSet::default();
      if !c.call_site.is_null()
        && let Some(module) = &self.module
      {
        // find_unique_types_exprs 已降 safe：集合/map 直传引用。
        // Safety: call_site 已在上方判空，args 元素指向模块 AST arena（cpp 同位置
        // 直接透传同一指针）。
        find_unique_types(
          &mut unique_types,
          unsafe { (*c.call_site).args.as_slice() },
          &module.ast_types,
        );
      }

      let resolution = resolver.resolve_overload(
        overload_to_use,
        args_pack,
        if c.call_site.is_null() {
          Location::default()
        } else {
          // Safety: else 分支即上方判空的结果，`c.call_site` 非空；
          // AstExprCall.func 由 parser 保证指向 AST arena 内的表达式节点
          // （cpp `c.callSite->func->location` 直接解引用）。
          unsafe { (*(*c.call_site).func).base.location }
        },
        &mut unique_types as *mut DenseHashSet<TypeId>,
        true,
      );

      let selected = resolution.get_unambiguous_overload();
      if let Some(overload) = selected.overload {
        overload_to_use = overload;
      } else {
        // 重载歧义/无解时把返回包绑 error（cpp 同分支）。
        let err_pack = self.builtin_types_ref().error_type_pack;
        self.bind_pack(constraint, c.result, err_pack);
        return true;
      }

      if resolution.metamethods.contains(&overload_to_use) {
        args_pack = self
          .arena_mut()
          .add_type_pack_vector_type_id_optional_type_pack_id(alloc::vec![fn_ty], Some(args_pack));
      }
    }

    let ret_tp = self.arena_mut().fresh_type_pack(scope, Polarity::Positive);
    track_interior_free_type_pack(scope, ret_tp);

    let inferred_ty = self.arena_mut().add_type(FunctionType::function_type_new(
      args_pack, ret_tp, None, false,
    ));

    let mut u2 = Unifier2::unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter(
            NonNull::new(self.arena.as_ptr()).expect(nc::HANDLE_AS_PTR),
            NonNull::new(self.builtin_types.as_ptr()).expect(nc::HANDLE_AS_PTR),
            NonNull::new(scope).expect(nc::SCOPE_IS_CONSTRAINT_SCOPE),
            NonNull::new(&self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter).expect(nc::REF_AS_PTR),
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
        self.normalizer.as_ptr(),
        self.type_function_runtime.as_ptr(),
        &self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter,
      );

      let mut has_bound = false;
      for (_, ty) in u2.generic_substitutions.iter() {
        if let Some(ftv) = get_type::get::<FreeType>(*ty) {
          let lower_bound = follow_type::follow(ftv.lower_bound);
          let upper_bound = follow_type::follow(ftv.upper_bound);
          has_bound = get_type::get::<NeverType>(lower_bound).is_none()
            || get_type::get::<UnknownType>(upper_bound).is_none();

          if has_bound {
            break;
          }
        }
      }

      if get_type::get::<FunctionType>(overload_to_use).is_some() && has_bound {
        let mut clone_state = CloneState {
          builtin_types: self.builtin_types,
          seen_types: DenseHashMap::default(),
          seen_type_packs: DenseHashMap::default(),
        };

        // Safety: `overload_to_use` 是 follow 后的 arena 节点（TypeId 非空），
        // `self.arena.get_mut()` 与 `clone_state` 均为本次持有的独占借用；
        // shallow_clone 的 unsafe 契约（源类型/arena/state 同代存活）在此满足。
        let cloned_ty = unsafe {
          shallow_clone(
            overload_to_use,
            self.arena.get_mut(),
            &mut clone_state,
            true,
          )
        };
        // C++ `LUAU_ASSERT(clonedFn)`：clone 保持 FunctionType 变体。
        let cloned_fn = get_mutable_type::get_mutable::<FunctionType>(cloned_ty)
          .expect("shallow_clone 保持 FunctionType 变体（cpp LUAU_ASSERT(clonedFn)）");
        cloned_fn.generics.clear();
        cloned_fn.generic_packs.clear();

        if let Some(subst) = instantiate_2_type_id(
          Handle::from_ptr(self.arena.as_ptr()),
          u2.generic_substitutions.clone(),
          u2.generic_pack_substitutions.clone(),
          &mut subtyping as *mut Subtyping,
          scope,
          cloned_ty,
        ) {
          overload_to_use = follow_type::follow(subst);

          if let Some(instantiated_fn) = get_type::get::<FunctionType>(overload_to_use) {
            result_tp = follow_type_pack::follow(instantiated_fn.ret_types);
          } else {
            self.report_error_type_error_data_location(CodeTooComplex::default().into(), &location);
            result_tp = self.builtin_types_ref().error_type_pack;
          }
        } else {
          self.report_error_type_error_data_location(CodeTooComplex::default().into(), &location);
          result_tp = self.builtin_types_ref().error_type_pack;
        }
      } else {
        let approximate_ret =
          get_approximate_return_type_for_function_call_type_id(overload_to_use)
            .unwrap_or(self.builtin_types_ref().error_type_pack);

        if let Some(subst) = instantiate_2_type_pack(
          Handle::from_ptr(self.arena.as_ptr()),
          u2.generic_substitutions.clone(),
          u2.generic_pack_substitutions.clone(),
          &mut subtyping as *mut Subtyping,
          scope,
          approximate_ret,
        ) {
          result_tp = subst;
        } else {
          self.report_error_type_error_data_location(CodeTooComplex::default().into(), &location);
          result_tp = self.builtin_types_ref().error_type_pack;
        }
      }
    }

    if !used_magic {
      // 非 magic 路径统一把推断出的 `result_tp` 绑到 `c.result`（cpp 同分支）。
      self.bind_pack(constraint, c.result, result_tp);
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
          // Safety: 两个指针均已在行首判非空——`ast_overload_resolved_types`
          // 指向模块级 DenseHashMap（cpp `(*c.astOverloadResolvedTypes)[c.callSite]`
          // 的 NotNull 语义），call_site 为 AST arena 内节点，写入键值不产生别名。
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

    if fflag::LuauIterativeInstantiationQueuer.get() {
      let mut queuer = InstantiationQueuer::new(
        NonNull::new(scope).expect(nc::SCOPE_IS_CONSTRAINT_SCOPE),
        &location,
        self as *mut ConstraintSolver,
      );
      queuer.run_type_id(overload_to_use);
      if fflag::LuauAlsoInstantiateInferredArguments.get() {
        queuer.run_type_pack_id(args_pack);
      }
      queuer.run_type_pack_id(result);
    } else {
      let mut queuer = InstantiationQueuerDeprecated::instantiation_queuer_deprecated_instantiation_queuer_deprecated(
                NonNull::new(scope).expect(nc::SCOPE_IS_CONSTRAINT_SCOPE),
                &location,
                self as *mut ConstraintSolver,
            );
      queuer.traverse_type_id(overload_to_use);
      if fflag::LuauAlsoInstantiateInferredArguments.get() {
        queuer.traverse_type_pack_id(args_pack);
      }
      queuer.traverse_type_pack_id(result);
    }

    if !fflag::LuauConstraintGraph.get() {
      self.unblock_type_pack_id_location(c.result, location);
    }

    true
  }

  pub fn try_dispatch_function_check_constraint_not_null_constraint_bool(
    &mut self,
    c: &FunctionCheckConstraint,
    constraint: &Constraint,
    mut force: bool,
  ) -> bool {
    if fflag::LuauForceLess.get() {
      // cpp: `if (FFlag::LuauForceLess) force = false;`
      force = false;
    }

    let fn_ty = follow_type::follow(c.fn_type);
    let args_pack = follow_type_pack::follow(c.args_pack);

    blocked_early_exit!(self, type fn_ty, constraint);

    if self.is_blocked_type_pack_id(args_pack) {
      return true;
    }

    // cpp: 仅在未开启 `LuauRelaxConstraintOrderingForFunctionCheck` 时做前置阻塞参数扫描
    if !fflag::LuauRelaxConstraintOrderingForFunctionCheck.get() {
      // Safety: `find_blocked_arg_types_in` 为 unsafe fn；cpp 在
      // tryDispatch(FunctionCheckConstraint) 中直接解引用 `c.callSite`
      // （ConstraintSolver.cpp:1915），即构造期保证非空的 AST 调用点指针，
      // `c.ast_types` 同为建约束时携带的模块级映射。
      let blocked_types = unsafe {
        find_blocked_arg_types_in(
          c.call_site,
          c.ast_types as *mut DenseHashMap<*const AstExpr, TypeId>,
        )
      };
      for ty in &blocked_types {
        self.block_type_id_not_null_constraint(*ty, constraint);
      }
      if !blocked_types.is_empty() {
        return false;
      }
    }

    let Some(ftv) = get_type::get::<FunctionType>(fn_ty) else {
      return true;
    };

    let mut replacements: DenseHashMap<TypeId, TypeId> = DenseHashMap::default();
    let mut replacement_packs: DenseHashMap<TypePackId, TypePackId> = DenseHashMap::default();

    let mut generic_types_and_packs: DenseHashSet<*const ()> = DenseHashSet::default();

    let mut u2 = Unifier2::unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter(
            NonNull::new(self.arena.as_ptr()).expect(nc::HANDLE_AS_PTR),
            NonNull::new(self.builtin_types.as_ptr()).expect(nc::HANDLE_AS_PTR),
            NonNull::new(constraint.scope).expect(nc::CONSTRAINT_SCOPE),
            NonNull::new(&self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter).expect(nc::REF_AS_PTR),
        );

    for generic in &ftv.generics {
      // We may see non-generic types here, for example when evaluating a
      // recursive function call.
      if let Some(gty) = get_type::get::<GenericType>(follow_type::follow(*generic)) {
        let repl_ty = if gty.polarity == Polarity::Negative {
          self.builtin_types_ref().never_type
        } else {
          self.builtin_types_ref().unknown_type
        };
        replacements.try_insert(*generic, repl_ty);
        generic_types_and_packs.insert_mut(*generic as *const ());
      }
    }

    for generic_pack in &ftv.generic_packs {
      replacement_packs.try_insert(*generic_pack, self.builtin_types_ref().unknown_type_pack);
      generic_types_and_packs.insert_mut(*generic_pack as *const ());
    }

    // cpp: `extendTypePack(*arena, builtinTypes, ftv->argTypes, callSite->args.size + typeOffset).head`
    // （把参数包补齐到调用点长度，而不是只取已有头部）
    // Safety: `c.call_site` 非空由 FunctionCheck 约束构造契约保证（cpp 对
    // `c.callSite` 全程直接解引用，无判空），指向模块 AST arena 的调用点，
    // 借出引用仅在本函数体内使用、只读。
    let call_site = unsafe { &*c.call_site };
    let args_slice = call_site.args.as_slice();
    let type_offset = if call_site.self_ { 1 } else { 0 };
    // Safety: `extend_type_pack` 为 unsafe fn（契约同 cpp `extendTypePack`）；
    // `self.arena.get_mut()`/`self.builtin_types.as_ptr()` 是两个不相交字段的构造期 NotNull
    // 指针，拆分借用避免与后续对 arena 的再次可变借用冲突。
    let expected_args = unsafe {
      extend_type_pack(
        self.arena.get_mut(),
        Handle::from_ptr(self.builtin_types.as_ptr()),
        ftv.arg_types,
        args_slice.len() + type_offset,
        Vec::new(),
      )
    }
    .head;
    let (arg_pack_head, _) = flatten_type_pack_id(args_pack);

    let subtyping =
      NonNull::new(self.subtyping).expect(nc::SUBTYPING);

    // 统一上界：原 break 条件只依赖 i，可提前算出可处理长度
    let bound = args_slice
      .len()
      .min(expected_args.len().saturating_sub(type_offset))
      .min(arg_pack_head.len().saturating_sub(type_offset));

    for (i, arg) in args_slice[..bound].iter().enumerate() {
      let expected_arg_ty = follow_type::follow(expected_args[i + type_offset]);
      let expr = unwrap_group(*arg);

      let result = push_type_into(
        NonNull::new(c.ast_types as *mut DenseHashMap<*const AstExpr, TypeId>)
          .expect("cpp pushTypeInto 首参为 NotNull：约束生成器为本约束接线非空 ast_types"),
        NonNull::new(c.ast_expected_types as *mut DenseHashMap<*const AstExpr, TypeId>)
          .expect("cpp pushTypeInto 次参为 NotNull：约束生成器为本约束接线非空 ast_expected_types"),
        NonNull::new(self as *mut ConstraintSolver).expect(nc::SELF_AS_PTR),
        NonNull::new(constraint as *const Constraint as *mut Constraint)
          .expect(nc::CONSTRAINT_REF),
        NonNull::new(&mut generic_types_and_packs as *mut DenseHashSet<*const ()>)
          .expect(nc::LOCAL_MUT_AS_PTR),
        NonNull::new(&mut u2 as *mut Unifier2).expect(nc::LOCAL_MUT_AS_PTR),
        subtyping,
        expected_arg_ty,
        expr as *const AstExpr,
      );

      if !force && !result.incomplete_types.is_empty() {
        for incomplete in &result.incomplete_types {
          let addition = self.push_constraint(
            NonNull::new(constraint.scope)
              .expect(nc::CONSTRAINT_SCOPE),
            constraint.location,
            ConstraintV::PushType(PushTypeConstraint {
              expected_type: incomplete.expected_type,
              target_type: incomplete.target_type,
              ast_types: c.ast_types,
              ast_expected_types: c.ast_expected_types,
              expr: incomplete.expr,
            }),
          );
          self.inherit_blocks(constraint, addition.as_ptr());
        }
      }
    }

    let incomplete_subtypes = u2.incomplete_subtypes.clone();
    for c_item in incomplete_subtypes {
      let addition = self.push_constraint(
        NonNull::new(constraint.scope)
          .expect(nc::CONSTRAINT_SCOPE),
        constraint.location,
        c_item,
      );
      self.inherit_blocks(constraint, addition.as_ptr());
    }

    true
  }

  pub(crate) fn try_dispatch_primitive_type_constraint_not_null_constraint(
    &mut self,
    c: &PrimitiveTypeConstraint,
    constraint: &Constraint,
  ) -> bool {
    let expected_type = c.expected_type.map(follow_type::follow);

    if let Some(et) = expected_type
      && (self.is_blocked_type_id(et) || get_type::get::<PendingExpansionType>(et).is_some())
    {
      return self.block_type_id_not_null_constraint(et, constraint);
    }

    let Some(free_type) = get_type::get::<FreeType>(follow_type::follow(c.free_type)) else {
      return true;
    };

    if fflag::LuauConstraintGraph.get() {
      if self
        .cgraph_mut()
        .has_strictly_more_than_one_dependency(BlockedConstraintId::V0(c.free_type))
      {
        self.block_type_id_not_null_constraint(c.free_type, constraint);
        return false;
      }
    } else {
      if let Some(it) = self.deprecated_type_to_constraint_set.get(&c.free_type)
        && it.len() > 1
      {
        self.block_type_id_not_null_constraint(c.free_type, constraint);
        return false;
      }
    }

    let mut bind_to = c.primitive_type;

    if free_type.upper_bound != c.primitive_type && maybe_singleton(free_type.upper_bound) {
      bind_to = free_type.lower_bound;
    } else if let Some(et) = expected_type
      && maybe_singleton(et)
    {
      bind_to = free_type.lower_bound;
    }

    let ty = follow_type::follow(c.free_type);
    if !fflag::LuauConstraintGraph.get() {
      self.deprecate_d_shift_references(ty, bind_to);
    }
    // 上方 `get_type::get::<FreeType>` 已证明 `ty` 仍是 FreeType、可绑定，与 cpp
    // `bind(constraint, ty, bindTo)` 前置一致。
    self.bind(constraint, ty, bind_to);

    true
  }

  pub(crate) fn try_dispatch_has_prop_constraint_not_null_constraint(
    &mut self,
    c: &HasPropConstraint,
    constraint: &Constraint,
  ) -> bool {
    let subject_type = follow_type::follow(c.subject_type);
    let result_type = follow_type::follow(c.result_type);

    LUAU_ASSERT!(get_type::get::<BlockedType>(result_type).is_some());
    LUAU_ASSERT!(can_mutate(result_type, constraint));

    blocked_early_exit!(self, type subject_type, constraint);

    if let Some(subject_table) = get_table_type(subject_type)
      && subject_table.state == TableState::Unsealed
      && subject_table.remaining_props > 0
      && !subject_table.props.contains_key(&c.prop)
    {
      return self.block_type_id_not_null_constraint(subject_type, constraint);
    }

    let lookup = self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool(
      constraint,
      subject_type,
      &c.prop,
      c.context,
      c.in_conditional,
      c.suppress_simplification,
    );
    if !lookup.blocked_types.is_empty() {
      for blocked in lookup.blocked_types {
        self.block_type_id_not_null_constraint(blocked, constraint);
      }
      return false;
    }

    // 函数开头两行 LUAU_ASSERT 已证 `result_type` 为 BlockedType 且 owner 正是本约束
    // （can_mutate）；查得 prop 类型后将其绑定（缺失则 any），对应 cpp hasProp 收尾 bind。
    let bound = lookup
      .prop_type
      .unwrap_or(self.builtin_types_ref().any_type);
    self.bind(constraint, result_type, bound);
    true
  }

  pub(crate) fn try_dispatch_has_indexer_constraint_not_null_constraint(
    &mut self,
    c: &HasIndexerConstraint,
    constraint: &Constraint,
  ) -> bool {
    let subject_type = follow_type::follow(c.subject_type);
    let index_type = follow_type::follow(c.index_type);

    blocked_early_exit!(self, type subject_type, constraint);
    blocked_early_exit!(self, type index_type, constraint);

    let mut btf = BlockedTypeFinder::blocked_type_finder_blocked_type_finder();
    btf.visit_type_id(subject_type);

    if let Some(blocked) = btf.blocked {
      return self.block_type_id_not_null_constraint(blocked, constraint);
    }

    let mut recursion_depth = 0;
    let mut seen = DenseHashSet::default();

    let result = self.constraint_solver_try_dispatch_has_indexer(
      &mut recursion_depth,
      constraint,
      subject_type,
      index_type,
      c.result_type,
      &mut seen,
    );

    if fflag::LuauConstraintGraph.get() && result {
      self.unblock_type_id_location(subject_type, Location::default());
    }

    result
  }

  pub fn try_dispatch_assign_prop_constraint_not_null_constraint(
    &mut self,
    c: &AssignPropConstraint,
    constraint: &Constraint,
  ) -> bool {
    let mut lhs_type = follow_type::follow(c.lhs_type);
    let rhs_type = follow_type::follow(c.rhs_type);

    blocked_early_exit!(self, type lhs_type, constraint);

    if let Some(lhs_extern_type) = get_type::get::<ExternType>(lhs_type) {
      let Some(prop) = lookup_extern_type_prop(lhs_extern_type, &c.prop_name) else {
        // Extern 类型查不到该属性 → 占位类型绑 any（cpp extern 分支）。
        let any_type = self.builtin_types_ref().any_type;
        self.bind(constraint, c.prop_type, any_type);
        return true;
      };

      if let Some(write_ty) = prop.write_ty {
        // 属性可写时占位类型绑 write 类型、再与 rhs 归并。
        self.bind(constraint, c.prop_type, write_ty);
        self.constraint_solver_unify(constraint, rhs_type, write_ty);
      } else {
        // 只读 extern 属性无 write 类型 → 退化为 any。
        let any_type = self.builtin_types_ref().any_type;
        self.bind(constraint, c.prop_type, any_type);
      }

      return true;
    }

    let lookup = self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool(
      constraint,
      lhs_type,
      &c.prop_name,
      ValueContext::LValue,
      false,
      false,
    );
    if !lookup.blocked_types.is_empty() {
      for blocked in lookup.blocked_types {
        self.block_type_id_not_null_constraint(blocked, constraint);
      }
      return false;
    }

    if let Some(prop_ty) = lookup.prop_type {
      let bound_prop_ty = if lookup.is_index {
        // SAFETY: arena/builtin_types 为不相交字段，须按裸指针拆分借用；均为
        // 构造期 NonNull、随 solver 存活，前者独占 add_type，后者只读 nil_type。
        {
          self.arena.get_mut().add_type(UnionType {
            options: alloc::vec![prop_ty, self.builtin_types.get().nil_type],
          })
        }
      } else {
        prop_ty
      };

      // 属性经 lookup 命中（非 indexer 时直接绑 prop 类型）。
      self.bind(constraint, c.prop_type, bound_prop_ty);
      self.constraint_solver_unify(constraint, rhs_type, prop_ty);
      return true;
    }

    let mut lhs_table = get_mutable_type::get_mutable::<TableType>(lhs_type);
    if lhs_table.is_none()
      && let Some(lhs_meta) = get_type::get::<MetatableType>(lhs_type)
    {
      lhs_type = follow_type::follow(lhs_meta.table);
      lhs_table = get_mutable_type::get_mutable::<TableType>(lhs_type);
    }

    if let Some(table) = lhs_table {
      if let Some(prop) = table.props.get_mut(&c.prop_name) {
        if let Some(write_ty) = prop.write_ty {
          // 具名属性已有 write 类型 → 占位直接绑它（cpp 同名分支）。
          self.bind(constraint, c.prop_type, write_ty);
          self.constraint_solver_unify(constraint, rhs_type, write_ty);
          return true;
        }

        if (table.state == TableState::Unsealed || table.state == TableState::Free)
          && prop.read_ty.is_some()
        {
          prop.write_ty = prop.read_ty;
          let write_ty = prop
            .write_ty
            .expect("上一行刚以 prop.read_ty（is_some() 守卫内）赋值 write_ty，必为 Some");
          // 未封存表允许把只读属性提升为可写后再绑占位。
          self.bind(constraint, c.prop_type, write_ty);
          self.constraint_solver_unify(constraint, rhs_type, write_ty);
          return true;
        }

        // 已封存表上的只读属性做写赋值 → 绑 error 表达类型错误。
        let error_type = self.builtin_types_ref().error_type;
        self.bind(constraint, c.prop_type, error_type);
        return true;
      }

      if let Some(indexer) = &table.indexer
        && maybe_string(indexer.index_type)
      {
        let prop_ty = indexer.index_result_type;
        // SAFETY: arena/builtin_types 为不相交字段，须按裸指针拆分借用。
        let union = {
          self.arena.get_mut().add_type(UnionType {
            options: alloc::vec![prop_ty, self.builtin_types.get().nil_type],
          })
        };
        // 字符串 indexer 命中 → 占位绑 index 结果与 nil 的 union。
        self.bind(constraint, c.prop_type, union);
        self.constraint_solver_unify(constraint, rhs_type, prop_ty);
        return true;
      }

      if table.state == TableState::Unsealed || table.state == TableState::Free {
        if fflag::LuauConstraintGraph.get() {
          LUAU_ASSERT!(!self.cgraph.is_null());
          self
            .cgraph_mut()
            .copy_dependencies_of_type_id(lhs_type, rhs_type);
        } else {
          self.deprecate_d_shift_references(lhs_type, rhs_type);
        }

        // 新属性路径先把依赖从 lhs 迁到 rhs，再让占位类型直接绑 `rhs_type`（cpp 同序）。
        self.bind(constraint, c.prop_type, rhs_type);

        let mut prop = Property::rw_type_id(rhs_type);
        prop.location = c.prop_location;
        table.props.insert(c.prop_name.clone(), prop);

        if table.state == TableState::Unsealed && c.decrement_prop_count {
          LUAU_ASSERT!(table.remaining_props > 0);
          table.remaining_props -= 1;
          self.unblock_type_id_location(lhs_type, constraint.location);
        }

        return true;
      }
    }

    let prop_type_is_blocked = get_type::get::<BlockedType>(c.prop_type).is_some();
    if prop_type_is_blocked {
      // 走到这里说明 lhs 不支持属性赋值：`c.prop_type` 是 BlockedType，可被其
      // owner（即本约束）改绑 error（cpp 尾分支同形）。
      let error_type = self.builtin_types_ref().error_type;
      self.bind(constraint, c.prop_type, error_type);
    }

    true
  }
}

fn table_stuff(
  solver: &mut ConstraintSolver,
  c: &AssignIndexConstraint,
  constraint: &Constraint,
  index_type: TypeId,
  rhs_type: TypeId,
  lhs_table: &mut TableType,
) -> Option<bool> {
  if let Some(indexer) = &lhs_table.indexer {
    solver.constraint_solver_unify(constraint, index_type, indexer.index_type);
    solver.constraint_solver_unify(constraint, rhs_type, indexer.index_result_type);

    // 索引赋值已有 indexer 时绑 index 结果∪nil（cpp 同分支）；`add_union` 的
    // arena/builtin_types 为不相交字段的构造期 NotNull 指针，拆分借用不产生别名。
    let bound = add_union(
      solver.arena,
      solver.builtin_types,
      &[
        indexer.index_result_type,
        solver.builtin_types.get().nil_type,
      ],
    );
    solver.bind(constraint, c.prop_type, bound);

    return Some(true);
  }

  if lhs_table.state == TableState::Unsealed || lhs_table.state == TableState::Free {
    lhs_table.indexer = Some(TableIndexer {
      index_type,
      index_result_type: rhs_type,
      is_read_only: false,
    });

    // 刚为表补上 indexer 后即把占位类型绑到 `rhs_type`（cpp 同序）。
    solver.bind(constraint, c.prop_type, rhs_type);
    return Some(true);
  }

  None
}

impl ConstraintSolver {
  pub fn try_dispatch_assign_index_constraint_not_null_constraint(
    &mut self,
    c: &AssignIndexConstraint,
    constraint: &Constraint,
  ) -> bool {
    let lhs_type: TypeId = follow_type::follow(c.lhs_type);
    let index_type: TypeId = follow_type::follow(c.index_type);
    let rhs_type: TypeId = follow_type::follow(c.rhs_type);

    blocked_early_exit!(self, type lhs_type, constraint);

    if let Some(lhs_free) = get_mutable_type::get_mutable::<FreeType>(lhs_type) {
      let lhs_upper = follow_type::follow(lhs_free.upper_bound);
      if let Some(lhs_table) = get_mutable_type::get_mutable::<TableType>(lhs_upper)
        && let Some(v) = table_stuff(self, c, constraint, index_type, rhs_type, lhs_table)
      {
        return v;
      }

      let new_upper_bound = self.arena_mut().add_type(
        TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
          &Default::default(),
          Some(TableIndexer {
            index_type,
            index_result_type: rhs_type,
            is_read_only: false,
          }),
          TypeLevel::default(),
          constraint.scope,
          TableState::Free,
        ),
      );

      self.constraint_solver_unify(constraint, lhs_type, new_upper_bound);

      // C++ `LUAU_ASSERT(newTable); LUAU_ASSERT(newTable->indexer);`
      let new_table = get_type::get::<TableType>(new_upper_bound)
        .expect("fresh_type 刚以 TableType 建入 arena，必命中");
      LUAU_ASSERT!(new_table.indexer.is_some());

      let idx_res = new_table
        .indexer
        .as_ref()
        .expect(
          "cpp LUAU_ASSERT(newTable->indexer)：fresh Free 表由 add_type 带 indexer 构造，必为 Some",
        )
        .index_result_type;
      self.bind(constraint, c.prop_type, idx_res);
      return true;
    }

    if let Some(lhs_table) = get_mutable_type::get_mutable::<TableType>(lhs_type)
      && let Some(v) = table_stuff(self, c, constraint, index_type, rhs_type, lhs_table)
    {
      return v;
    }

    // C++ `if (auto lhsExternType = get<ExternType>(lhsType))`：仅 lhs 为
    // ExternType 时进入，循环退出后 return true。
    if let Some(first_et) = get_type::get::<ExternType>(lhs_type) {
      let mut et = Some(first_et);
      while let Some(cur) = et {
        if let Some(indexer) = &cur.indexer {
          self.constraint_solver_unify(constraint, index_type, indexer.index_type);
          self.constraint_solver_unify(constraint, rhs_type, indexer.index_result_type);

          let res_ty = add_union(
            self.arena,
            self.builtin_types,
            &[indexer.index_result_type, self.builtin_types_ref().nil_type],
          );
          // Extern 父链查到 indexer 后绑其结果∪nil。
          self.bind(constraint, c.prop_type, res_ty);
          return true;
        }

        et = cur.parent.and_then(get_type::get::<ExternType>);
      }
      return true;
    }

    if let Some(lhs_intersection) = get_mutable_type::get_mutable::<IntersectionType>(lhs_type) {
      let mut parts = TypeIds::new();

      // C++ `for (TypeId t : lhsIntersection)` — IntersectionTypeIterator 防环展平
      // 并 follow Bound,裸遍历 parts 会漏掉嵌套 intersection。
      for t in begin_intersection_type(lhs_intersection) {
        let followed = follow_type::follow(t);

        if let Some(tbl) = get_mutable_type::get_mutable::<TableType>(followed) {
          if let Some(indexer) = &tbl.indexer {
            self.constraint_solver_unify(constraint, index_type, indexer.index_type);
            parts.insert_type_id(indexer.index_result_type);
          }

          if tbl.state == TableState::Unsealed || tbl.state == TableState::Free {
            tbl.indexer = Some(TableIndexer {
              index_type,
              index_result_type: rhs_type,
              is_read_only: false,
            });
            parts.insert_type_id(rhs_type);
          }

          continue;
        }

        let mut cls = get_type::get::<ExternType>(followed);
        while let Some(et) = cls {
          if let Some(indexer) = &et.indexer {
            self.constraint_solver_unify(constraint, index_type, indexer.index_type);
            parts.insert_type_id(indexer.index_result_type);
            break;
          }

          cls = et.parent.and_then(get_type::get::<ExternType>);
        }
      }

      let res = self.simplify_intersection_not_null_scope_location_type_ids(
        constraint.scope,
        constraint.location,
        parts,
      );
      self.constraint_solver_unify(constraint, rhs_type, res);
    }

    // Other types do not support index assignment：把不支持索引赋值的占位类型绑 error。
    let error_type = self.builtin_types_ref().error_type;
    self.bind(constraint, c.prop_type, error_type);

    true
  }

  pub fn try_dispatch_unpack_constraint_not_null_constraint(
    &mut self,
    c: &UnpackConstraint,
    constraint: &Constraint,
  ) -> bool {
    let source_pack = follow_type_pack::follow(c.source_pack);

    blocked_early_exit!(self, pack source_pack, constraint);

    // Safety: `extend_type_pack` 是 unsafe fn（契约同 cpp `extendTypePack`：
    // 包指针为 arena 内活 TypePackId）；`self.arena.get_mut()` 独占 arena 分配、
    // `self.builtin_types.as_ptr()` 为不相交字段的构造期 NotNull 指针。
    let src_pack = unsafe {
      extend_type_pack(
        self.arena.get_mut(),
        Handle::from_ptr(self.builtin_types.as_ptr()),
        source_pack,
        c.result_pack.len(),
        Vec::new(),
      )
    };

    // 源包头部与结果槽位逐位配对；zip 天然止于二者较短长度（cpp `i` 上界）。
    let paired = src_pack.head.len().min(c.result_pack.len());
    for (&src, &result) in src_pack.head.iter().zip(c.result_pack.iter()) {
      let src_ty = follow_type::follow(src);
      let result_ty = follow_type::follow(result);

      if get_type::get::<BlockedType>(result_ty).is_some() {
        LUAU_ASSERT!(can_mutate_type_id(result_ty, constraint));

        if follow_type::follow(src_ty) == result_ty {
          let scope = constraint.scope;
          let fresh_ty = fresh_type(
            // SAFETY: arena 在 solver 存活期内有效。
            { self.arena.get_mut() },
            // SAFETY: builtin_types 在 solver 存活期内有效。
            { self.builtin_types.get() },
            scope,
            Polarity::Positive,
          );
          track_interior_free_type(scope, fresh_ty);

          if fflag::LuauConstraintGraph.get() {
            // `result_ty` 与 `src_ty` 自反相等，只能改绑新 fresh 类型避免自环。
            self.bind(constraint, result_ty, fresh_ty);
          } else {
            self.deprecate_d_shift_references(result_ty, fresh_ty);
            // SAFETY: `result_ty` 刚被证明是 arena 内的 BlockedType，
            // `as_mutable_type_id` 仅做 const→mut 指针转换；手写 Bound 与
            // bind() 内部 emplace 同构，写后不再经该节点取共享引用。
            unsafe {
              (*as_mutable_type_id(result_ty)).ty = TypeVariant::Bound(fresh_ty);
            }
          }
        } else {
          // 常规解包路径——把源类型直接绑到结果槽位。
          self.bind(constraint, result_ty, src_ty);
        }
      } else {
        self.constraint_solver_unify(constraint, src_ty, result_ty);
      }

      if !fflag::LuauConstraintGraph.get() {
        self.unblock_type_id_location(result_ty, constraint.location);
      }
    }

    // 源包耗尽后多余的结果槽位：blocked/PE 一律绑 nil（cpp 同分支）。
    let nil_type = self.builtin_types_ref().nil_type;
    for result in c.result_pack[paired..].iter() {
      let result_ty = follow_type::follow(*result);
      LUAU_ASSERT!(can_mutate_type_id(result_ty, constraint));

      if get_type::get::<BlockedType>(result_ty).is_some()
        || get_type::get::<PendingExpansionType>(result_ty).is_some()
      {
        self.bind(constraint, result_ty, nil_type);
      }
    }

    true
  }
}

fn can_mutate_type_id(ty: TypeId, constraint: *const Constraint) -> bool {
  if let Some(blocked) = get_type::get::<BlockedType>(ty) {
    let owner = blocked.get_owner();
    LUAU_ASSERT!(!owner.is_null());
    return eq(owner, constraint);
  }

  true
}

impl ConstraintSolver {
  pub fn try_dispatch_reduce_constraint_not_null_constraint_bool(
    &mut self,
    c: &ReduceConstraint,
    constraint: &Constraint,
    force: bool,
  ) -> bool {
    let mut ty = follow_type::follow(c.ty);

    let scope = constraint.scope;
    let location = constraint.location;

    let mut context = TypeFunctionContext::from_solver(
      NonNull::new(self as *mut ConstraintSolver).expect(nc::SELF_AS_PTR),
      NonNull::new(scope).expect(nc::SCOPE_IS_CONSTRAINT_SCOPE),
      NonNull::new(constraint as *const Constraint as *mut Constraint)
        .expect(nc::CONSTRAINT_REF),
      NonNull::new(self.subtyping).expect(nc::SUBTYPING),
    );
    let mut result = reduce_type_functions(ty, location, &mut context, force);

    for r in result.reduced_types.iter() {
      self.unblock_type_id_location(*r, location);
    }

    for r in result.reduced_packs.iter() {
      self.unblock_type_pack_id_location(*r, location);
    }

    for ity in result.irreducible_types.iter() {
      self.uninhabited_type_functions.insert(*ity as *const ());
      self.unblock_type_id_location(*ity, location);
    }

    let reduction_finished = result.blocked_types.empty() && result.blocked_packs.empty();

    ty = follow_type::follow(ty);

    // If we couldn't reduce this type function, stick it in the set!
    if get_type::get::<TypeFunctionInstanceType>(ty).is_some()
      && result.irreducible_types.find(&ty).is_none()
    {
      *self.type_functions_to_finalize.get_or_insert(ty) = constraint as *const Constraint;
    }

    if force || reduction_finished {
      for message in take(&mut result.messages) {
        self.report_error_type_error(message);
      }

      // if we're completely dispatching this constraint, we want to record any uninhabited type functions to unblock.
      for error in result.errors.iter() {
        if let Some(utf) = get_type_error::<UninhabitedTypeFunction>(error) {
          self.uninhabited_type_functions.insert(utf.ty as *const ());
        } else if let Some(utpf) = get_type_error::<UninhabitedTypePackFunction>(error) {
          self.uninhabited_type_functions.insert(utpf.tp as *const ());
        }
      }
    }

    if force {
      return true;
    }

    for b in result.blocked_types.iter() {
      self.block_type_id_not_null_constraint(*b, constraint);
    }

    for b in result.blocked_packs.iter() {
      self.block_type_pack_id_not_null_constraint(*b, constraint);
    }

    reduction_finished
  }

  pub fn try_dispatch_reduce_pack_constraint_not_null_constraint_bool(
    &mut self,
    c: &ReducePackConstraint,
    constraint: &Constraint,
    mut force: bool,
  ) -> bool {
    if fflag::LuauForceLess.get() {
      // cpp: `if (FFlag::LuauForceLess) force = false;`
      force = false;
    }

    let tp = follow_type_pack::follow(c.tp);

    let scope = constraint.scope;
    let location = constraint.location;

    let mut context = TypeFunctionContext::from_solver(
      NonNull::new(self as *mut ConstraintSolver).expect(nc::SELF_AS_PTR),
      NonNull::new(scope).expect(nc::SCOPE_IS_CONSTRAINT_SCOPE),
      NonNull::new(constraint as *const Constraint as *mut Constraint)
        .expect(nc::CONSTRAINT_REF),
      NonNull::new(self.subtyping).expect(nc::SUBTYPING),
    );
    let result = reduce_type_functions_tp(tp, location, &mut context, force);

    for r in result.reduced_types.iter() {
      self.unblock_type_id_location(*r, location);
    }

    for r in result.reduced_packs.iter() {
      self.unblock_type_pack_id_location(*r, location);
    }

    let reduction_finished = result.blocked_types.empty() && result.blocked_packs.empty();

    if force || reduction_finished {
      // if we're completely dispatching this constraint, we want to record any uninhabited type functions to unblock.
      for error in result.errors.iter() {
        if let Some(utf) = get_type_error::<UninhabitedTypeFunction>(error) {
          self.uninhabited_type_functions.insert(utf.ty as *const ());
        } else if let Some(utpf) = get_type_error::<UninhabitedTypePackFunction>(error) {
          self.uninhabited_type_functions.insert(utpf.tp as *const ());
        }
      }
    }

    if force {
      return true;
    }

    for b in result.blocked_types.iter() {
      self.block_type_id_not_null_constraint(*b, constraint);
    }

    for b in result.blocked_packs.iter() {
      self.block_type_pack_id_not_null_constraint(*b, constraint);
    }

    reduction_finished
  }

  pub fn try_dispatch_equality_constraint_not_null_constraint(
    &mut self,
    c: &EqualityConstraint,
    constraint: &Constraint,
  ) -> bool {
    self.constraint_solver_unify(constraint, c.result_type, c.assignment_type);
    self.constraint_solver_unify(constraint, c.assignment_type, c.result_type);
    true
  }

  pub fn try_dispatch_simplify_constraint_not_null_constraint_bool(
    &mut self,
    c: &SimplifyConstraint,
    constraint: &Constraint,
    force: bool,
  ) -> bool {
    let scope = constraint.scope;
    let location = constraint.location;
    let target = follow_type::follow(c.ty);

    // cpp simplify 前置守卫：持久/他 arena 或非 union 目标直接返回。
    if type_is_foreign_or_persistent(target, self.arena.get().arena_id)
      || get_type::get::<UnionType>(target).is_none()
    {
      return true;
    }

    let mut finder = FindAllUnionMembers::new();
    finder.traverse_type_id(target);

    if !finder.blocked_tys.empty() && !force {
      for ty in &finder.blocked_tys.order {
        self.block_type_id_not_null_constraint(*ty, constraint);
      }
      return false;
    }

    let mut result = self.builtin_types_ref().never_type;
    for ty in &finder.recorded_tys.order {
      let ty_followed = follow_type::follow(*ty);
      if ty_followed == target {
        continue;
      }
      result = self.simplify_union(scope, location, result, ty_followed);
    }

    if force {
      for ty in &finder.blocked_tys.order {
        let ty_followed = follow_type::follow(*ty);
        if ty_followed == target {
          continue;
        }
        result = self.simplify_union(scope, location, result, ty_followed);
      }
    }

    let mutable_target = { as_mutable_type_id(target) };
    let mut result_arg = result;
    // Safety: `mutable_target` 由 `as_mutable_type_id` 从非空 `target` 转换而来，
    // 指向本 arena 的 union 宿主节点；遍历器已结束、`result` 是 Copy 指针，
    // 此刻无其它活跃借用，独占改写 Bound 视图与 cpp `getMutable(t)->ty` 同构。
    unifiable_bound_type_id_emplace_type_bound_type(
      unsafe { &mut *mutable_target },
      &mut result_arg,
    );

    // Rust 移植在 flag 关闭时 cgraph 为空（走 deprecate_d 路径），故保留守卫；
    // 对应 C++ 无条件 `cgraph->shiftReferences(target, result)`（ConstraintSolver.cpp:2994）。
    if fflag::LuauConstraintGraph.get() {
      self.cgraph_mut().shift_references_type_id(target, result);
    }

    true
  }

  pub(crate) fn try_dispatch_push_function_type_constraint_not_null_constraint(
    &mut self,
    c: &PushFunctionTypeConstraint,
    constraint: &Constraint,
  ) -> bool {
    let Some(mut expected_fn) =
      get_type::get::<FunctionType>(follow_type::follow(c.expected_function_type))
    else {
      return true;
    };
    let Some(fn_ty) = get_type::get::<FunctionType>(follow_type::follow(c.function_type)) else {
      // 若期望类型或给定类型不是函数，直接 bail。
      return true;
    };

    // cpp: `instantiate(builtinTypes, arena, NotNull{&limits}, constraint->scope, c.expectedFunctionType)`
    let instantiated = instantiate(
      // Safety: `instantiate` 是安全函数，unsafe 只来自拆分借用：
      // `self.builtin_types.get()`（只读）与 `self.arena.get_mut()`（独占）是 `self`
      // 上不相交两字段的裸指针重借用，均为构造期 NonNull、随 solver 存活；
      // 若改用 `builtin_types_ref()`/`arena_mut()` 访问器会互相冲突，
      // 这正是本文件保留裸指针拆分借用的原因。
      { self.builtin_types.get() },
      // Safety: 上方拆分借用的另一半——`self.arena.get_mut()` 为构造期 NonNull、
      // 随 solver 存活，与 builtin_types 的只读借用不相交，instantiate 期间
      // 无其他 arena 引用。
      { self.arena.get_mut() },
      &self.limits,
      constraint.scope,
      c.expected_function_type,
    );
    let Some(instantiated) = instantiated else {
      // cpp: 实例化失败，直接 bail。
      return true;
    };
    // cpp: `LUAU_ASSERT(expectedFn)` — 实例化结果必仍是函数类型。
    let new_expected_fn = get_type::get::<FunctionType>(follow_type::follow(instantiated));
    LUAU_ASSERT!(new_expected_fn.is_some());
    if let Some(t) = new_expected_fn {
      expected_fn = t;
    }

    let mut expected_params = begin(expected_fn.arg_types);
    let mut params = begin(fn_ty.arg_types);

    let expected_params_end = end_type_pack_id(expected_fn.arg_types);
    let params_end = end_type_pack_id(fn_ty.arg_types);

    if expected_params == expected_params_end || params == params_end {
      return true;
    }

    if c.is_self {
      let params_current = *params.current();
      if get_type::get::<FreeType>(follow_type::follow(params_current)).is_some() {
        // self 形参位无注解且为自由类型时绑期望签名首参（cpp 同分支）。
        let expected_current = *expected_params.current();
        self.bind(constraint, params_current, expected_current);
      }
      expected_params.advance();
      params.advance();
    }

    // Safety: `c.expr` 是该 PushFunction 约束对应的 AstExprFunction——cpp
    // tryDispatch(PushFunctionTypeConstraint) 全程直接解引用 `c.expr`
    // （ConstraintSolver.cpp:2968 起），构造契约保证非空；AST arena 存活
    // 至整次求解结束，借出的引用仅在本函数内只读使用。
    let expr = unsafe { &*c.expr };
    let args = expr.args.as_slice();
    // `idx` 是附加 `AstExprFunction` 参数的索引；存在 `self` 时无需对参数偏移。
    for arg in args {
      if expected_params == expected_params_end || params == params_end {
        break;
      }

      let annotation = arg.annotation;
      let params_current = *params.current();

      // 注解优先于一切，见到注解就 bail；非自由类型同样不在推断范围内。
      if annotation.is_null()
        && get_type::get::<FreeType>(follow_type::follow(params_current)).is_some()
      {
        // 与 is_self 分支同构——把无注解的自由形参绑到期望类型。
        let expected_current = *expected_params.current();
        self.bind(constraint, params_current, expected_current);
      }

      expected_params.advance();
      params.advance();
    }

    if expr.return_annotation.is_null()
      && get_type_pack::get::<FreeTypePack>(fn_ty.ret_types).is_some()
    {
      // 返回注解缺失且返回包自由 → 绑到期望函数返回包。
      self.bind_pack(constraint, fn_ty.ret_types, expected_fn.ret_types);
    }

    true
  }

  pub fn try_dispatch_type_instantiation_constraint_not_null_constraint(
    &mut self,
    c: &TypeInstantiationConstraint,
    constraint: &Constraint,
  ) -> bool {
    LUAU_ASSERT!(LuauExplicitTypeInstantiationSupport.get());

    blocked_early_exit!(self, type c.function_type, constraint);

    let bound_to = self.instantiate_function_type(
      c.function_type,
      &c.type_arguments,
      &c.type_pack_arguments,
      constraint.scope,
      &constraint.location,
    );
    // 显式类型实例化占位（placeholder_type）在约束创建时由 arena fresh 出、专供本约束绑定。
    self.bind(constraint, c.placeholder_type, bound_to);

    true
  }

  pub fn try_dispatch_push_type_constraint_not_null_constraint_bool(
    &mut self,
    c: &PushTypeConstraint,
    constraint: &Constraint,
    mut force: bool,
  ) -> bool {
    if fflag::LuauForceLess.get() {
      // cpp: `if (FFlag::LuauForceLess) force = false;`
      force = false;
    }

    let mut u2 = Unifier2::unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter_dense_hash_set_void(
            NonNull::new(self.arena.as_ptr()).expect(nc::HANDLE_AS_PTR),
            NonNull::new(self.builtin_types.as_ptr()).expect(nc::HANDLE_AS_PTR),
            NonNull::new(constraint.scope).expect(nc::CONSTRAINT_SCOPE),
            NonNull::new(&self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter).expect(nc::REF_AS_PTR),
            &mut self.uninhabited_type_functions as *mut DenseHashSet<*const ()>,
        );

    let subtyping =
      NonNull::new(self.subtyping).expect(nc::SUBTYPING);

    // NOTE: If we don't do this check up front, we almost immediately start
    // spawning tons of push type constraints. It's pretty important.
    if self.is_blocked_type_id(c.expected_type) {
      self.block_type_id_not_null_constraint(c.expected_type, constraint);
      // If we're forcing this constraint and the expected type is blocked, we
      // should just bail.
      return force;
    }

    let mut empty: DenseHashSet<*const ()> = DenseHashSet::default();
    let result = push_type_into(
      NonNull::new(c.ast_types as *mut DenseHashMap<*const AstExpr, TypeId>)
        .expect("cpp pushTypeInto 首参为 NotNull：约束生成器为本约束接线非空 ast_types"),
      NonNull::new(c.ast_expected_types as *mut DenseHashMap<*const AstExpr, TypeId>)
        .expect("cpp pushTypeInto 次参为 NotNull：约束生成器为本约束接线非空 ast_expected_types"),
      NonNull::new(self as *mut ConstraintSolver).expect(nc::SELF_AS_PTR),
      NonNull::new(constraint as *const Constraint as *mut Constraint)
        .expect(nc::CONSTRAINT_REF),
      NonNull::new(&mut empty as *mut DenseHashSet<*const ()>).expect(nc::LOCAL_MUT_AS_PTR),
      NonNull::new(&mut u2 as *mut Unifier2).expect(nc::LOCAL_MUT_AS_PTR),
      subtyping,
      c.expected_type,
      c.expr,
    );

    // If we're forcing this constraint, just early exit: we can continue
    // inferring the rest of the file, we might just error when we shouldn't.
    if force || result.incomplete_types.is_empty() {
      return true;
    }

    for incomplete in &result.incomplete_types {
      let addition = self.push_constraint(
        NonNull::new(constraint.scope)
          .expect(nc::CONSTRAINT_SCOPE),
        constraint.location,
        ConstraintV::PushType(PushTypeConstraint {
          expected_type: incomplete.expected_type,
          target_type: incomplete.target_type,
          ast_types: c.ast_types,
          ast_expected_types: c.ast_expected_types,
          expr: incomplete.expr,
        }),
      );
      self.inherit_blocks(constraint, addition.as_ptr());
    }

    true
  }
}
