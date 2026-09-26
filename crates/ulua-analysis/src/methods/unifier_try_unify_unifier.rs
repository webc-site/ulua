use alloc::string::String;
use core::{
  mem::swap,
  ptr::{null, null_mut},
};

use ulua_common::fint;

use crate::{
  enums::{normalization_result::NormalizationResult, polarity::Polarity, variance::Variance},
  functions::{
    flatten_type_pack::flatten,
    fresh_type::fresh_type,
    get_type,
    is_blocked_unifier::{is_blocked_txn_log_type_id, is_blocked_txn_log_type_pack_id},
    is_optional::is_optional,
    is_prim::is_prim,
    promote_type_levels_unifier::promote_type_levels_txn_log_type_arena_type_level_type_id,
    size_type_pack::size,
  },
  records::{
    any_type::AnyType,
    arena_handle::{alias_opt, alias_ref},
    arena_id::ArenaId,
    count_mismatch::{CountMismatch, CountMismatchContext},
    extern_type::ExternType,
    free_type::FreeType,
    free_type_pack::FreeTypePack,
    function_type::FunctionType,
    generic_error::GenericError,
    generic_type::GenericType,
    intersection_type::IntersectionType,
    metatable_type::MetatableType,
    negation_type::NegationType,
    never_type::NeverType,
    normalization_too_complex::NormalizationTooComplex,
    primitive_type::{PrimitiveType, Type as PrimType},
    scope::Scope,
    singleton_type::SingletonType,
    table_type::TableType,
    r#type::Type,
    type_function_instance_type::TypeFunctionInstanceType,
    type_level::TypeLevel,
    type_pack::TypePack,
    type_pack_mismatch::TypePackMismatch,
    type_pack_var::TypePackVar,
    unification_too_complex::UnificationTooComplex,
    unifier::Unifier,
    union_type::UnionType,
    unknown_type::UnknownType,
    variadic_type_pack::VariadicTypePack,
    weird_iter::WeirdIter,
    widen::Widen,
  },
  type_aliases::{
    error_type::ErrorType, error_type_pack::ErrorTypePack, literal_properties::LiteralProperties,
    type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant, type_variant::TypeVariant,
  },
};

impl Unifier {
  /// `void Unifier::tryUnify(TypeId sub_ty, TypeId super_ty, bool isFunctionCall, bool isIntersection, const LiteralProperties* literalProperties)`
  pub fn try_unify_type_id_type_id_bool_bool_literal_properties_entry(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    is_function_call: bool,
    is_intersection: bool,
    literal_properties: Option<&LiteralProperties>,
  ) {
    self.shared_state_mut().counters.iteration_count = 0;

    self.try_unify_type_id_type_id_bool_bool_literal_properties(
      sub_ty,
      super_ty,
      is_function_call,
      is_intersection,
      literal_properties,
    );
  }

  /// `void Unifier::tryUnify_(TypeId sub_ty, TypeId super_ty, bool isFunctionCall, bool isIntersection, const LiteralProperties* literalProperties)`
  pub fn try_unify_type_id_type_id_bool_bool_literal_properties(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    is_function_call: bool,
    is_intersection: bool,
    literal_properties: Option<&LiteralProperties>,
  ) {
    self.try_unify_impl(
      sub_ty,
      super_ty,
      is_function_call,
      is_intersection,
      literal_properties,
    )
  }

  // 内部实现：裸指针解引用由 unsafe 块承担（私有可见性，不触发签名契约告警）。
  fn try_unify_impl(
    &mut self,
    mut sub_ty: TypeId,
    mut super_ty: TypeId,
    is_function_call: bool,
    is_intersection: bool,
    literal_properties: Option<&LiteralProperties>,
  ) {
    {
      let counters = &mut self.shared_state_mut().counters;
      counters.iteration_count += 1;
      if counters.iteration_limit > 0 && counters.iteration_limit < counters.iteration_count {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
        );
        return;
      }
    }

    super_ty = self.log.follow_type_id(super_ty);
    sub_ty = self.log.follow_type_id(sub_ty);

    if super_ty == sub_ty {
      return;
    }

    // Reflexive structural-equality fast-path. C++ relies on alias-shared
    // TypeIds making the `super_ty == sub_ty` pointer check above fire
    // pervasively; we don't pointer-share alias-derived composites, so
    // structurally-identical unions/functions/packs (e.g. `Color <: Color`,
    // Color = "red" | "blue") otherwise re-run the full walk on every
    // curried use and blow the iteration limit (np_hard). Reflexive — sound.
    if self.reflexive_equal_type_id(super_ty, sub_ty, 32) {
      return;
    }

    let sub_blocked = is_blocked_txn_log_type_id(&self.log, sub_ty);
    let super_blocked = is_blocked_txn_log_type_id(&self.log, super_ty);
    if sub_blocked && super_blocked {
      self.blocked_types.push(sub_ty);
      self.blocked_types.push(super_ty);
    } else if sub_blocked {
      self.blocked_types.push(sub_ty);
    } else if super_blocked {
      self.blocked_types.push(super_ty);
    }

    if self
      .log
      .txn_log_get::<TypeFunctionInstanceType, TypeId>(super_ty)
      .is_some()
    {
      self.ice_string("Unexpected TypeFunctionInstanceType super_ty");
    }

    if self
      .log
      .txn_log_get::<TypeFunctionInstanceType, TypeId>(sub_ty)
      .is_some()
    {
      self.ice_string("Unexpected TypeFunctionInstanceType sub_ty");
    }

    let super_free = self.log.txn_log_get_mutable::<FreeType, TypeId>(super_ty);
    let sub_free = self.log.txn_log_get_mutable::<FreeType, TypeId>(sub_ty);

    // C++ `subsumes(a, b)` for free/generic vars: `a->level.subsumes(b->level)`.
    // (Caller guarantees non-null pointers at each use site.)
    // super_free/sub_free 是 txn_log_get_mutable::<FreeType,_> 的快照：alias_opt
    // 门面收口「null→None、非 null→arena 稳定块/pending 副本的存活节点」（§2），
    // 以下仅读 level 比较，与原 `unsafe { p.as_ref() }` 逐格等价。
    let super_ft_opt = alias_opt(super_free);
    let sub_ft_opt = alias_opt(sub_free);
    if let (Some(super_ft), Some(sub_ft)) = (super_ft_opt, sub_ft_opt)
      && super_ft.level.subsumes(&sub_ft.level)
    {
      if !self.occurs_check_type_id_type_id_bool(sub_ty, super_ty, false) {
        self
          .log
          .replace_type_id_t(sub_ty, Type::new(TypeVariant::Bound(super_ty)));
      }
      return;
    } else if let (Some(super_ft), Some(sub_ft)) = (super_ft_opt, sub_ft_opt) {
      // 原 `!super_free.is_null() && !sub_free.is_null()` 哨兵即 alias_opt 的
      // Some 判据；进入时首分支的 subsumes 已判否，行为不变。
      if !self.occurs_check_type_id_type_id_bool(super_ty, sub_ty, true) {
        if super_ft.level.subsumes(&sub_ft.level) {
          self
            .log
            .change_level_type_id_type_level(sub_ty, super_ft.level);
        }
        self
          .log
          .replace_type_id_t(super_ty, Type::new(TypeVariant::Bound(sub_ty)));
      }
      return;
    } else if let Some(super_ft) = super_ft_opt {
      // 上方分支未命中：至多一侧是 Free。super_ft 复用函数开头对 super_free
      // 快照的受证成 Option（null 时为 None，本分支整体不进入）。
      // Unification can't change the level of a generic.
      // 只读 level 判定走 txn_log_get（Option<&T> 安全面，§2），不再有裸指针快照。
      let sub_generic_opt = self.log.txn_log_get::<GenericType, TypeId>(sub_ty);
      if let Some(sub_gt) = sub_generic_opt
        && !sub_gt.level.subsumes(&super_ft.level)
      {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::GenericError(GenericError::new(String::from(
            "Generic subtype escaping scope",
          ))),
        );
        return;
      }

      if !self.occurs_check_type_id_type_id_bool(super_ty, sub_ty, true) {
        let super_level = super_ft.level;
        // promote_type_levels_* 已 Rust 化为 safe fn（入参引用、arena 归属读
        // 收口在 alias_ref 门面），本调用点无 unsafe。
        promote_type_levels_txn_log_type_arena_type_level_type_id(
          &mut self.log,
          self.types.get(),
          super_level,
          sub_ty,
        );

        let mut widen = Widen::widen_widen(self.types, self.builtin_types);
        let widened = widen.widen_type(sub_ty);
        self
          .log
          .replace_type_id_t(super_ty, Type::new(TypeVariant::Bound(widened)));
      }
      return;
    } else if let Some(sub_ft) = sub_ft_opt {
      // 与 super 侧对称：sub_ft 复用开头对 sub_free 快照的受证成 Option，
      // null 时本分支不进入；命中即指向 arena/log 中稳定存活的 FreeType 节点。
      // Normally, if the subtype is free, it should not be bound to any, unknown, or error types.
      // But for bug compatibility, we'll only apply this rule to unknown.
      if self
        .log
        .txn_log_get::<UnknownType, TypeId>(super_ty)
        .is_some()
      {
        return;
      }

      // 与 sub 侧对称：只读 level 判定走 txn_log_get 安全面（§2）。
      let super_generic_opt = self.log.txn_log_get::<GenericType, TypeId>(super_ty);
      if let Some(super_gt) = super_generic_opt
        && !super_gt.level.subsumes(&sub_ft.level)
      {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::GenericError(GenericError::new(String::from(
            "Generic supertype escaping scope",
          ))),
        );
        return;
      }

      if !self.occurs_check_type_id_type_id_bool(sub_ty, super_ty, false) {
        let sub_level = sub_ft.level;
        // 同 super 侧：promote_type_levels_* 为 safe fn，直接调用。
        promote_type_levels_txn_log_type_arena_type_level_type_id(
          &mut self.log,
          self.types.get(),
          sub_level,
          super_ty,
        );
        self
          .log
          .replace_type_id_t(sub_ty, Type::new(TypeVariant::Bound(super_ty)));
      }
      return;
    }

    if self.log.txn_log_get::<AnyType, TypeId>(super_ty).is_some() {
      return self.try_unify_with_any_type_id_type_id(sub_ty, self.builtin_types_ref().any_type);
    }

    if self.log.txn_log_get::<AnyType, TypeId>(sub_ty).is_some() {
      if self.normalize {
        // TODO: there are probably cheaper ways to check if any <: T.
        match self.normalizer_mut().try_normalize(super_ty) {
          // 归一化 tops 不是 Any：any <: T 不成立，仅置 failure（后续统一走 tryUnifyWithAny）
          Some(super_norm) if get_type::get::<AnyType>(super_norm.tops).is_none() => {
            self.failure = true;
          }
          Some(_) => {}
          // 归一化过于复杂：与 C++ 一致报错返回
          None => {
            self.report_error_location_type_error_data(
              self.location,
              TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
            );
            return;
          }
        }
      } else {
        self.failure = true;
      }
      return self.try_unify_with_any_type_id_type_id(super_ty, self.builtin_types_ref().any_type);
    }

    if self.log.txn_log_get::<NeverType, TypeId>(sub_ty).is_some() {
      return self
        .try_unify_with_any_type_id_type_id(super_ty, self.builtin_types_ref().never_type);
    }

    // What if the types are immutable and we proved their relation before
    let cache_enabled =
      !is_function_call && !is_intersection && self.variance == Variance::Invariant;

    if cache_enabled {
      if self
        .shared_state_ref()
        .cached_unify
        .find(&(sub_ty, super_ty))
        .is_some()
      {
        return;
      }

      // C++: `if (auto error = sharedState.cachedUnifyError.find({sub_ty, super_ty})) { reportError(*error); return; }`
      // Serve a previously-cached unification error rather than re-exploring the
      // (potentially exponential) failing subtree. This negative-cache fast-path is
      // what keeps pathological intersection subtyping (e.g. graph-coloring encodings)
      // under the iteration limit.
      if let Some(error) = self
        .shared_state_ref()
        .cached_unify_error
        .find(&(sub_ty, super_ty))
        .cloned()
      {
        self.report_error_location_type_error_data(self.location, error);
        return;
      }
    }

    // If we have seen this pair before, we are recursing into cyclic types; assume they unify.
    if self.log.have_seen_type_id_type_id(super_ty, sub_ty) {
      return;
    }

    self.log.push_seen_type_id_type_id(super_ty, sub_ty);

    let error_count = self.errors.len();

    let sub_union = self.log.txn_log_get_mutable::<UnionType, TypeId>(sub_ty);
    let super_intersection = self
      .log
      .txn_log_get_mutable::<IntersectionType, TypeId>(super_ty);
    let super_union = self.log.txn_log_get_mutable::<UnionType, TypeId>(super_ty);
    let sub_intersection = self
      .log
      .txn_log_get_mutable::<IntersectionType, TypeId>(sub_ty);

    if let Some(sub_union_ref) = alias_opt(sub_union) {
      // sub_union 源自上方 txn_log_get_mutable::<UnionType,_>(sub_ty)：alias_opt
      // 门面把「null→None、非 null→arena/pending 稳定节点共享借用」收口到一处
      // （§2），Some 判定与原 `!is_null()` 分支逐格等价；被调方签名已 Rust 化为
      // `&'static UnionType`（C++ `const UnionType*` 同契约，只读 options 列表）。
      self.unifier_try_unify_union_with_type(sub_ty, sub_union_ref, super_ty);
    } else if let Some(super_intersection_ref) = alias_opt(super_intersection) {
      // super_intersection 快照经 alias_opt 收口（null→None 与原 `!is_null()`
      // 分支等价）；被调方签名已 Rust 化为 `&'static IntersectionType`
      // （C++ `tryUnifyTypeWithIntersection` 的只读 `const IntersectionType*`）。
      self.unifier_try_unify_type_with_intersection(sub_ty, super_ty, super_intersection_ref);
    } else if let Some(super_union_ref) = alias_opt(super_union) {
      // super_union 由上文 txn_log_get_mutable::<UnionType,_>(super_ty) 产生，
      // alias_opt 折叠 null→None（Some 判定与原 `!is_null()` 逐格等价），非 null
      // 指向 super_ty 的稳定 UnionType 节点；被调方签名已 Rust 化为
      // `&'static UnionType`（C++ `tryUnifyTypeWithUnion(..., const UnionType* uv, ...)` 只读）。
      self.unifier_try_unify_type_with_union(
        sub_ty,
        super_ty,
        super_union_ref,
        cache_enabled,
        is_function_call,
      );
    } else if let Some(sub_intersection_ref) = alias_opt(sub_intersection) {
      // sub_intersection 快照同源（getMutable 族，稳定地址），alias_opt 折叠
      // null→None 与原 `!is_null()` 逐格等价；被调方签名已 Rust 化为
      // `&'static IntersectionType`（C++ `tryUnifyIntersectionWithType` 只读 parts）。
      self.unifier_try_unify_intersection_with_type(
        sub_ty,
        sub_intersection_ref,
        super_ty,
        cache_enabled,
        is_function_call,
      );
    } else if self.log.txn_log_get::<AnyType, TypeId>(sub_ty).is_some() {
      self.try_unify_with_any_type_id_type_id(super_ty, self.builtin_types_ref().unknown_type);
      self.failure = true;
    } else if self.log.txn_log_get::<ErrorType, TypeId>(sub_ty).is_some()
      && self
        .log
        .txn_log_get::<ErrorType, TypeId>(super_ty)
        .is_some()
    {
      // error <: error
    } else if self
      .log
      .txn_log_get::<ErrorType, TypeId>(super_ty)
      .is_some()
    {
      self.try_unify_with_any_type_id_type_id(sub_ty, self.builtin_types_ref().error_type);
      self.failure = true;
    } else if self.log.txn_log_get::<ErrorType, TypeId>(sub_ty).is_some() {
      self.try_unify_with_any_type_id_type_id(super_ty, self.builtin_types_ref().error_type);
      self.failure = true;
    } else if self
      .log
      .txn_log_get::<UnknownType, TypeId>(super_ty)
      .is_some()
    {
      // At this point, all the supertypes of `error` have been handled.
      self.try_unify_with_any_type_id_type_id(sub_ty, self.builtin_types_ref().unknown_type);
    } else if self
      .log
      .txn_log_get::<PrimitiveType, TypeId>(super_ty)
      .is_some()
      && self
        .log
        .txn_log_get::<PrimitiveType, TypeId>(sub_ty)
        .is_some()
    {
      self.unifier_try_unify_primitives(sub_ty, super_ty);
    } else if (self
      .log
      .txn_log_get::<PrimitiveType, TypeId>(super_ty)
      .is_some()
      || self
        .log
        .txn_log_get::<SingletonType, TypeId>(super_ty)
        .is_some())
      && self
        .log
        .txn_log_get::<SingletonType, TypeId>(sub_ty)
        .is_some()
    {
      self.unifier_try_unify_singletons(sub_ty, super_ty);
    } else if get_type::get::<PrimitiveType>(super_ty)
      .is_some_and(|p| p.r#type == PrimType::Function)
      && get_type::get::<FunctionType>(sub_ty).is_some()
    {
      // Ok. Do nothing. forall functions F, F <: function
    } else if is_prim(super_ty, PrimType::Table)
      && (get_type::get::<TableType>(sub_ty).is_some()
        || get_type::get::<MetatableType>(sub_ty).is_some())
    {
      // Ok, do nothing: forall tables T, T <: table
    } else if self
      .log
      .txn_log_get::<FunctionType, TypeId>(super_ty)
      .is_some()
      && self
        .log
        .txn_log_get::<FunctionType, TypeId>(sub_ty)
        .is_some()
    {
      self.unifier_try_unify_functions(sub_ty, super_ty, is_function_call);
    } else if self
      .log
      .txn_log_get::<PrimitiveType, TypeId>(super_ty)
      .is_some_and(|table| table.r#type == PrimType::Table)
    {
      let empty_table = self.builtin_types_ref().empty_table_type;
      self.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
        sub_ty,
        empty_table,
        is_function_call,
        is_intersection,
        None,
      );
    } else if self
      .log
      .txn_log_get::<PrimitiveType, TypeId>(sub_ty)
      .is_some_and(|table| table.r#type == PrimType::Table)
    {
      let empty_table = self.builtin_types_ref().empty_table_type;
      self.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
        empty_table,
        super_ty,
        is_function_call,
        is_intersection,
        None,
      );
    } else if self
      .log
      .txn_log_get::<TableType, TypeId>(super_ty)
      .is_some()
      && self.log.txn_log_get::<TableType, TypeId>(sub_ty).is_some()
    {
      // SAFETY: 进入本分支 ⇒ 上文两侧 txn_log_get::<TableType> 均为 Some，即
      // sub_ty/super_ty 都是 TableType 句柄，满足被调方非 table 即 ice 的前置条件；
      // literal_properties
      // 源自本函数参数 `Option<&LiteralProperties>`——lp 指针在其借用（整个 try_unify
      // 同步调用期）内有效，None 时传 null，被调方按 C++ `const LiteralProperties*`
      // 契约判空使用。
      unsafe {
        self.unifier_try_unify_tables(
          sub_ty,
          super_ty,
          is_intersection,
          literal_properties.map_or(null(), |lp| lp as *const LiteralProperties),
        )
      };
    } else if self
      .log
      .txn_log_get::<TableType, TypeId>(super_ty)
      .is_some()
      && (self
        .log
        .txn_log_get::<PrimitiveType, TypeId>(sub_ty)
        .is_some()
        || self
          .log
          .txn_log_get::<SingletonType, TypeId>(sub_ty)
          .is_some())
    {
      self.unifier_try_unify_scalar_shape(sub_ty, super_ty, false);
    } else if self.log.txn_log_get::<TableType, TypeId>(sub_ty).is_some()
      && (self
        .log
        .txn_log_get::<PrimitiveType, TypeId>(super_ty)
        .is_some()
        || self
          .log
          .txn_log_get::<SingletonType, TypeId>(super_ty)
          .is_some())
    {
      self.unifier_try_unify_scalar_shape(sub_ty, super_ty, true);
    } else if self
      .log
      .txn_log_get::<MetatableType, TypeId>(super_ty)
      .is_some()
    {
      self.unifier_try_unify_with_metatable(sub_ty, super_ty, false);
    } else if self
      .log
      .txn_log_get::<MetatableType, TypeId>(sub_ty)
      .is_some()
    {
      self.unifier_try_unify_with_metatable(super_ty, sub_ty, true);
    } else if self
      .log
      .txn_log_get::<ExternType, TypeId>(super_ty)
      .is_some()
    {
      self.unifier_try_unify_with_extern_type(sub_ty, super_ty, false);
    } else if self.log.txn_log_get::<ExternType, TypeId>(sub_ty).is_some() {
      self.unifier_try_unify_with_extern_type(sub_ty, super_ty, true);
    } else if self
      .log
      .txn_log_get::<NegationType, TypeId>(super_ty)
      .is_some()
      || self
        .log
        .txn_log_get::<NegationType, TypeId>(sub_ty)
        .is_some()
    {
      self.unifier_try_unify_negations(sub_ty, super_ty);
    } else if self.check_inhabited
      && self.normalizer_mut().is_inhabited_type_id(sub_ty) == NormalizationResult::False
    {
      // uninhabited; nothing to do
    } else {
      self.unifier_report_type_mismatch(super_ty, sub_ty);
    }

    if cache_enabled {
      self.unifier_cache_result(sub_ty, super_ty, error_count);
    }

    self.log.pop_seen_type_id_type_id(super_ty, sub_ty);
  }

  /// `void Unifier::tryUnify(TypePackId subTp, TypePackId superTp, bool isFunctionCall)`
  pub fn try_unify_type_pack_id_type_pack_id_bool_entry(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    is_function_call: bool,
  ) {
    self.shared_state_mut().counters.iteration_count = 0;

    self.try_unify_type_pack_id_type_pack_id_bool(sub_tp, super_tp, is_function_call);
  }

  /// `void Unifier::tryUnify_(TypePackId subTp, TypePackId superTp, bool isFunctionCall)`
  pub fn try_unify_type_pack_id_type_pack_id_bool(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    is_function_call: bool,
  ) {
    self.try_unify_pack_impl(sub_tp, super_tp, is_function_call)
  }

  // 内部实现：裸指针解引用由 unsafe 块承担（私有可见性，不触发签名契约告警）。
  fn try_unify_pack_impl(
    &mut self,
    mut sub_tp: TypePackId,
    mut super_tp: TypePackId,
    is_function_call: bool,
  ) {
    {
      let counters = &mut self.shared_state_mut().counters;
      counters.iteration_count += 1;
      if counters.iteration_limit > 0 && counters.iteration_limit < counters.iteration_count {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
        );
        return;
      }
    }

    super_tp = self.log.follow_type_pack_id(super_tp);
    sub_tp = self.log.follow_type_pack_id(sub_tp);

    // Reflexive structural-equality fast-path (see unifier_reflexive_equal):
    // identical curried-function arg/return packs (e.g. `(Color)` vs
    // `(Color)`) recur element-by-element on every use without this, which
    // is what tips np_hard over the iteration limit.
    if self.reflexive_equal_type_pack_id(super_tp, sub_tp, 32) {
      return;
    }

    while let Some(tp) = alias_opt(self.log.txn_log_get_mutable::<TypePack, TypePackId>(sub_tp)) {
      // alias_opt 门面收口 txn_log_get_mutable 快照（null→None ≡ 原 is_null break，
      // 非 null→arena/pending 稳定 TypePack 节点），以下仅读 head/tail（§2）。
      if !tp.head.is_empty() {
        break;
      }
      let Some(tail) = tp.tail else {
        break;
      };
      sub_tp = self.log.follow_type_pack_id(tail);
    }

    // 与 sub 侧同构：alias_opt 折叠快照，仅读 head/tail（§2）。
    while let Some(tp) = alias_opt(
      self
        .log
        .txn_log_get_mutable::<TypePack, TypePackId>(super_tp),
    ) {
      if !tp.head.is_empty() {
        break;
      }
      let Some(tail) = tp.tail else {
        break;
      };
      super_tp = self.log.follow_type_pack_id(tail);
    }

    if super_tp == sub_tp {
      return;
    }

    if self
      .log
      .have_seen_type_pack_id_type_pack_id(super_tp, sub_tp)
    {
      return;
    }

    let sub_blocked = is_blocked_txn_log_type_pack_id(&self.log, sub_tp);
    let super_blocked = is_blocked_txn_log_type_pack_id(&self.log, super_tp);
    if sub_blocked && super_blocked {
      self.blocked_type_packs.push(sub_tp);
      self.blocked_type_packs.push(super_tp);
    } else if sub_blocked {
      self.blocked_type_packs.push(sub_tp);
    } else if super_blocked {
      self.blocked_type_packs.push(super_tp);
    }

    if !self
      .log
      .txn_log_get_mutable::<FreeTypePack, TypePackId>(super_tp)
      .is_null()
    {
      if !self.occurs_check_type_pack_id_type_pack_id_bool(super_tp, sub_tp, true) {
        let mut widen = Widen::widen_widen(self.types, self.builtin_types);
        let widened = widen.widen_type_pack(sub_tp);
        let bound = TypePackVar {
          ty: TypePackVariant::Bound(widened),
          persistent: false,
          owning_arena: ArenaId::NONE,
        };
        self.log.replace_type_pack_id_type_pack_var(super_tp, bound);
      }
    } else if !self
      .log
      .txn_log_get_mutable::<FreeTypePack, TypePackId>(sub_tp)
      .is_null()
    {
      if !self.occurs_check_type_pack_id_type_pack_id_bool(sub_tp, super_tp, false) {
        let bound = TypePackVar {
          ty: TypePackVariant::Bound(super_tp),
          persistent: false,
          owning_arena: ArenaId::NONE,
        };
        self.log.replace_type_pack_id_type_pack_var(sub_tp, bound);
      }
    } else if !self
      .log
      .txn_log_get_mutable::<ErrorTypePack, TypePackId>(super_tp)
      .is_null()
    {
      self.try_unify_with_any_type_pack_id_type_pack_id(sub_tp, super_tp);
    } else if !self
      .log
      .txn_log_get_mutable::<ErrorTypePack, TypePackId>(sub_tp)
      .is_null()
    {
      self.try_unify_with_any_type_pack_id_type_pack_id(super_tp, sub_tp);
    } else if !self
      .log
      .txn_log_get_mutable::<VariadicTypePack, TypePackId>(super_tp)
      .is_null()
    {
      self.unifier_try_unify_variadics(sub_tp, super_tp, false, 0);
    } else if !self
      .log
      .txn_log_get_mutable::<VariadicTypePack, TypePackId>(sub_tp)
      .is_null()
    {
      self.unifier_try_unify_variadics(super_tp, sub_tp, true, 0);
    } else if !self
      .log
      .txn_log_get_mutable::<TypePack, TypePackId>(super_tp)
      .is_null()
      && !self
        .log
        .txn_log_get_mutable::<TypePack, TypePackId>(sub_tp)
        .is_null()
    {
      // 快照经 alias_opt 门面折叠（null→None ≡ 原 getMutable 指针判空，同路径同
      // 地址；§2 收口），并在 flatten/iter 改写 log 之前即刻拷出 tail 句柄——
      // 与原 C++ 「getMutable 后马上读 tail」的时序逐格一致。
      let snap_super_tail = alias_opt(
        self
          .log
          .txn_log_get_mutable::<TypePack, TypePackId>(super_tp),
      )
      .and_then(|p| p.tail);
      let snap_sub_tail = alias_opt(self.log.txn_log_get_mutable::<TypePack, TypePackId>(sub_tp))
        .and_then(|p| p.tail);

      // If the size of two heads does not match, but both packs have free tail
      // we set the sentinel to avoid growing forever.
      let (super_types, super_tail) = flatten(super_tp, &self.log);
      let (sub_types, sub_tail) = flatten(sub_tp, &self.log);

      let no_infinite_growth = (super_types.len() != sub_types.len())
        && super_tail.is_some_and(|t| {
          !self
            .log
            .txn_log_get_mutable::<FreeTypePack, TypePackId>(t)
            .is_null()
        })
        && sub_tail.is_some_and(|t| {
          !self
            .log
            .txn_log_get_mutable::<FreeTypePack, TypePackId>(t)
            .is_null()
        });

      let mut super_iter = WeirdIter {
        pack_id: super_tp,
        log: &mut self.log as *mut _,
        // 占位：紧随的 init 调用第一句即覆盖为 txn_log 查得的槽位指针，其间无读取
        // （C 型冗余哨兵；字段本体为 `*mut TypePack`，收口在 records/weird_iter.rs）。
        pack: null_mut(),
        index: 0,
        growing: false,
        level: TypeLevel::default(),
        // 原「先置空再由下方补写」的哨兵已删（C 型）：init 方法不改 scope，
        // 直接以最终值接线。
        scope: self.scope.as_ptr(),
      };
      super_iter.weird_iter_type_pack_id_txn_log(super_tp, &mut self.log);

      let mut sub_iter = WeirdIter {
        pack_id: sub_tp,
        log: &mut self.log as *mut _,
        pack: null_mut(),
        index: 0,
        growing: false,
        level: TypeLevel::default(),
        scope: self.scope.as_ptr(),
      };
      sub_iter.weird_iter_type_pack_id_txn_log(sub_tp, &mut self.log);

      let empty_tp = self.types_mut().add_type_pack_t(TypePack::empty());

      let mut loop_count = 0;

      loop {
        if fint::LuauTypeInferTypePackLoopLimit.get() > 0
          && loop_count >= fint::LuauTypeInferTypePackLoopLimit.get()
        {
          self.ice_string("Detected possibly infinite TypePack growth");
        }

        loop_count += 1;

        if super_iter.weird_iter_good() && sub_iter.growing {
          let ft = self.mk_fresh_for_iter(sub_iter.scope);
          sub_iter.weird_iter_push_type(ft);
        }

        if sub_iter.weird_iter_good() && super_iter.growing {
          let ft = self.mk_fresh_for_iter(super_iter.scope);
          super_iter.weird_iter_push_type(ft);
        }

        if super_iter.weird_iter_good() && sub_iter.weird_iter_good() {
          let s = *sub_iter.current();
          let sup = *super_iter.current();
          self.try_unify_type_id_type_id_bool_bool_literal_properties(s, sup, false, false, None);

          if !self.errors.is_empty() && self.first_pack_error_pos.is_none() {
            self.first_pack_error_pos = Some(loop_count);
          }

          super_iter.weird_iter_advance();
          sub_iter.weird_iter_advance();
          continue;
        }

        // If both are at the end, we're done
        if !super_iter.weird_iter_good() && !sub_iter.weird_iter_good() {
          // snap_super_tail/snap_sub_tail 是分支入口 alias_opt 快照即刻拷出的句柄，
          // 此处仅对 follow 后的 id 查询 free pack（§2，无解引用）。
          let l_free_tail = snap_super_tail.is_some_and(|t| {
            !self
              .log
              .txn_log_get_mutable::<FreeTypePack, TypePackId>(self.log.follow_type_pack_id(t))
              .is_null()
          });
          let r_free_tail = snap_sub_tail.is_some_and(|t| {
            !self
              .log
              .txn_log_get_mutable::<FreeTypePack, TypePackId>(self.log.follow_type_pack_id(t))
              .is_null()
          });
          if l_free_tail && r_free_tail {
            // l/r_free_tail 各自蕴含对应 tail 为 Some。
            self.try_unify_type_pack_id_type_pack_id_bool(
              snap_sub_tail.expect("l_free_tail 谓词由上方 tail-free 判定蕴含 sub_tail 为 Some"),
              snap_super_tail
                .expect("r_free_tail 谓词由上方 tail-free 判定蕴含 super_tail 为 Some"),
              false,
            );
          } else if l_free_tail {
            // l_free_tail 蕴含 super_tail 为 Some。
            self.try_unify_type_pack_id_type_pack_id_bool(
              empty_tp,
              snap_super_tail.expect("l_free_tail 蕴含 super_tail 为 Some"),
              false,
            );
          } else if r_free_tail {
            // r_free_tail 蕴含 sub_tail 为 Some。
            self.try_unify_type_pack_id_type_pack_id_bool(
              empty_tp,
              snap_sub_tail.expect("r_free_tail 蕴含 sub_tail 为 Some"),
              false,
            );
          } else if snap_sub_tail.is_some() && snap_super_tail.is_some() {
            if !self
              .log
              .txn_log_get_mutable::<VariadicTypePack, TypePackId>(super_iter.pack_id)
              .is_null()
            {
              self.unifier_try_unify_variadics(
                sub_iter.pack_id,
                super_iter.pack_id,
                false,
                sub_iter.index as i32,
              );
            } else if !self
              .log
              .txn_log_get_mutable::<VariadicTypePack, TypePackId>(sub_iter.pack_id)
              .is_null()
            {
              self.unifier_try_unify_variadics(
                super_iter.pack_id,
                sub_iter.pack_id,
                true,
                super_iter.index as i32,
              );
            } else {
              self.try_unify_type_pack_id_type_pack_id_bool(
                snap_sub_tail
                  .expect("外层 snap_sub_tail.is_some() && snap_super_tail.is_some() 守卫必中"),
                snap_super_tail
                  .expect("外层 snap_sub_tail.is_some() && snap_super_tail.is_some() 守卫必中"),
                false,
              );
            }
          }

          break;
        }

        // If both tails are free, bind one to the other and call it a day
        if super_iter.weird_iter_can_grow() && sub_iter.weird_iter_can_grow() {
          // WeirdIter::pack 由 init 恒指向存活 head 型 TypePack 节点（arena 块
          // 地址稳定）；alias_ref 门面收口解引用（§2），can_grow 蕴含 tail 为 Some。
          let s = alias_ref(sub_iter.pack)
            .tail
            .expect("weird_iter_can_grow 蕴含 pack.tail 为该 free pack（Some）");
          let sup = alias_ref(super_iter.pack)
            .tail
            .expect("weird_iter_can_grow 蕴含 pack.tail 为该 free pack（Some）");
          return self.try_unify_type_pack_id_type_pack_id_bool(s, sup, false);
        }

        // If just one side is free on its tail, grow it to fit the other side.
        if super_iter.weird_iter_can_grow() {
          let new_tail = self.types_mut().add_type_pack_type_pack_var(TypePackVar {
            ty: TypePackVariant::TypePack(TypePack::empty()),
            persistent: false,
            owning_arena: ArenaId::NONE,
          });
          super_iter.weird_iter_grow(new_tail);
        } else if sub_iter.weird_iter_can_grow() {
          let new_tail = self.types_mut().add_type_pack_type_pack_var(TypePackVar {
            ty: TypePackVariant::TypePack(TypePack::empty()),
            persistent: false,
            owning_arena: ArenaId::NONE,
          });
          sub_iter.weird_iter_grow(new_tail);
        } else {
          // A union type including nil marks an optional argument
          if super_iter.weird_iter_good() && is_optional(*super_iter.current()) {
            super_iter.weird_iter_advance();
            continue;
          } else if sub_iter.weird_iter_good() && is_optional(*sub_iter.current()) {
            sub_iter.weird_iter_advance();
            continue;
          }

          if !self
            .log
            .txn_log_get_mutable::<VariadicTypePack, TypePackId>(super_iter.pack_id)
            .is_null()
          {
            self.unifier_try_unify_variadics(
              sub_iter.pack_id,
              super_iter.pack_id,
              false,
              sub_iter.index as i32,
            );
            return;
          }

          if !self
            .log
            .txn_log_get_mutable::<VariadicTypePack, TypePackId>(sub_iter.pack_id)
            .is_null()
          {
            self.unifier_try_unify_variadics(
              super_iter.pack_id,
              sub_iter.pack_id,
              true,
              super_iter.index as i32,
            );
            return;
          }

          if !is_function_call && sub_iter.weird_iter_good() {
            // Sometimes it is ok to pass too many arguments
            return;
          }

          // This is a bit weird because we don't actually know expected vs actual.
          // size 已 Rust 化：C++ `TxnLog*` 形参 → Option<&TxnLog> 共享借用（§2）。
          let mut expected_size = size(super_tp, Some(&self.log));
          let mut actual_size = size(sub_tp, Some(&self.log));
          if self.ctx == CountMismatchContext::FunctionResult
            || self.ctx == CountMismatchContext::ExprListResult
          {
            swap(&mut expected_size, &mut actual_size);
          }
          let ctx = self.ctx;
          self.report_error_location_type_error_data(
            self.location,
            TypeErrorData::CountMismatch(CountMismatch {
              expected: expected_size,
              maximum: None,
              actual: actual_size,
              context: ctx,
              is_variadic: false,
              function: String::new(),
            }),
          );

          let error_type = self.builtin_types_ref().error_type;
          while super_iter.weird_iter_good() {
            let cur = *super_iter.current();
            self.try_unify_type_id_type_id_bool_bool_literal_properties(
              cur, error_type, false, false, None,
            );
            super_iter.weird_iter_advance();
          }

          while sub_iter.weird_iter_good() {
            let cur = *sub_iter.current();
            self.try_unify_type_id_type_id_bool_bool_literal_properties(
              cur, error_type, false, false, None,
            );
            sub_iter.weird_iter_advance();
          }

          return;
        }

        if no_infinite_growth {
          break;
        }
      }
    } else {
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::TypePackMismatch(TypePackMismatch {
          wanted_tp: super_tp,
          given_tp: sub_tp,
          reason: String::new(),
        }),
      );
    }
  }

  /// `mkFreshType` lambda in tryUnify_: `freshType(NotNull{types}, builtinTypes, scope)`.
  fn mk_fresh_for_iter(&mut self, scope: *mut Scope) -> TypeId {
    fresh_type(
      // arena 句柄解引用（契约见 records/arena_handle.rs）：本帧仅此一处
      // 可变使用；Handle 物化的借用不受借用检查器约束，与原裸指针语义同构。
      self.types.get_mut(),
      self.builtin_types_ref(),
      scope,
      Polarity::Positive,
    )
  }
}
