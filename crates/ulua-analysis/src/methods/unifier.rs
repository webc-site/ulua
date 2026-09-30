//! `unifier` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};

use ulua_ast::records::location::Location;

use crate::{
  enums::{context_error::Context, variance::Variance},
  functions::{
    get_singleton_type::get_singleton_type, get_type,
    has_unification_too_complex::has_unification_too_complex,
  },
  records::{
    arena_handle::Handle, boolean_singleton::BooleanSingleton, builtin_types::BuiltinTypes,
    intersection_type::IntersectionType, negation_type::NegationType,
    normalization_too_complex::NormalizationTooComplex, normalizer::Normalizer,
    primitive_type::PrimitiveType, scope::Scope, singleton_type::SingletonType,
    skip_cache_for_type::SkipCacheForType, string_singleton::StringSingleton, txn_log::TxnLog,
    type_arena::TypeArena, type_error::TypeError, type_mismatch::TypeMismatch, unifier::Unifier,
    unifier_shared_state::UnifierSharedState,
  },
  type_aliases::{
    collections::HashSet, error_vec::ErrorVec, type_error_data::TypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

// `Unifier` 上下文字段（`Handle` 句柄，见 `records/arena_handle.rs`）的
// 访问器门面：`builtin_types` / `normalizer` / `shared_state` 由构造方以
// `NonNull` 语义建立（C++ `NotNull<T>` / 引用同契约），unifier 存续期内恒有效。
// 解引用安全性已收拢进 `Handle` 单点，本文件不再出现 `unsafe`。

impl Unifier {
  /// C++ `NotNull<BuiltinTypes>`：内建类型表随前端上下文存活。
  pub fn builtin_types_ref(&self) -> &BuiltinTypes {
    self.builtin_types.get()
  }

  /// 归一化器：unifier 独占使用期间的可变访问（`try_normalize` 等带缓存写入）。
  pub fn normalizer_mut(&mut self) -> &mut Normalizer {
    self.normalizer.get_mut()
  }

  /// 类型 arena：unifier 独占期间的可变访问（新增 pack 等）。
  pub fn types_mut(&mut self) -> &mut TypeArena {
    self.types.get_mut()
  }

  /// C++ `NotNull<Scope*>`：当前 unifier 绑定的 scope，随求解上下文存活。
  pub fn scope_ref(&self) -> &Scope {
    self.scope.get()
  }

  /// C++ `UnifierSharedState&`：跨 unifier 共享的只读视图（缓存查找等）。
  pub fn shared_state_ref(&self) -> &UnifierSharedState {
    self.shared_state.get()
  }

  /// 共享状态的可变视图（计数器累加等）：与 C++ 相同，多个 unifier
  /// 顺序共享同一状态，单线程求解器内无并发别名。
  pub fn shared_state_mut(&mut self) -> &mut UnifierSharedState {
    self.shared_state.get_mut()
  }
}

impl Unifier {
  pub fn unifier_cache_result(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    prev_error_count: usize,
  ) {
    if self.errors.len() == prev_error_count {
      if self.unifier_can_cache_result(sub_ty, super_ty) {
        // shared_state 为 `Handle` 单例句柄（契约见 records/arena_handle.rs），
        // insert 的临时可变借用止于语句末。
        self
          .shared_state
          .get_mut()
          .cached_unify
          .insert((sub_ty, super_ty));
      }
    } else if self.errors.len() == prev_error_count + 1
      && self.unifier_can_cache_result(sub_ty, super_ty)
    {
      // C++: `sharedState.cachedUnifyError[{sub_ty, super_ty}] = errors.back().data;`
      // 链上 `errors.len() == prev_error_count + 1` 蕴含 errors 非空。
      let error_data = self
        .errors
        .last()
        .expect("链上 len()==prev+1>=1 判定蕴含非空")
        .data
        .clone();
      // 同一 shared_state 句柄契约；get_or_insert 与覆写的临时可变借用
      // 只覆盖本条赋值语句，error_data 是刚 clone 的本地拥有值。
      *self
        .shared_state
        .get_mut()
        .cached_unify_error
        .get_or_insert((sub_ty, super_ty)) = error_data;
    }
  }
}

impl Unifier {
  pub fn unifier_can_cache_result(&mut self, sub_ty: TypeId, super_ty: TypeId) -> bool {
    // shared_state 为 `Handle` 单例句柄（契约见 records/arena_handle.rs）：
    // 指向 TypeCheckResult 持有的共享状态、比本 Unifier 长寿；此处取共享
    // 借用只读 `skip_cache_for_type` 缓存表，单线程无并发写。
    let shared_state = self.shared_state.get();

    if let Some(super_ty_info) = shared_state.skip_cache_for_type.find(&super_ty)
      && *super_ty_info
    {
      return false;
    }

    if let Some(sub_ty_info) = shared_state.skip_cache_for_type.find(&sub_ty)
      && *sub_ty_info
    {
      return false;
    }

    let skip_cache_for = |ty: TypeId| -> bool {
      let mut visitor = SkipCacheForType::skip_cache_for_type_skip_cache_for_type(
        &shared_state.skip_cache_for_type,
        self.types.get().arena_id,
      );
      // C++ `visitor.traverse(ty)` — dispatch to the per-variant visit
      // overrides and recurse into composite types, so any nested mutable
      // element (unsealed/free table, free/bound/generic/blocked pack,
      // etc.) flips `result` and makes the unification uncacheable.
      let mut seen_types = HashSet::new();
      let mut seen_packs = HashSet::new();
      // 变体读取已收口于 `type_variant_of`，traverse 为 safe fn；`ty` 及其可达
      // 子节点均出自构造 visitor 时存入的 `self.types` arena（bump 分块、地址
      // 稳定，canCacheResult 全程不改写类型图）。
      visitor.traverse_skip_cache(ty, &mut seen_types, &mut seen_packs);

      // 重建可变视图——上方共享借用此刻只余 `find` 只读用途（读-改-写序列在
      // 单线程序列化，两个借用窗口不交叠），此借用仅把 visitor 结果记入
      // `skip_cache_for_type[ty]`，随即随闭包返回而丢弃。
      let mut_shared_state = self.shared_state.get_mut();
      mut_shared_state
        .skip_cache_for_type
        .try_insert(ty, visitor.result);
      visitor.result
    };

    if shared_state.skip_cache_for_type.find(&super_ty).is_none() && skip_cache_for(super_ty) {
      return false;
    }

    if shared_state.skip_cache_for_type.find(&sub_ty).is_none() && skip_cache_for(sub_ty) {
      return false;
    }

    true
  }
}

impl Unifier {
  pub fn can_unify_type_id_type_id(&mut self, sub_ty: TypeId, super_ty: TypeId) -> ErrorVec {
    let mut s = self.unifier_make_child_unifier();
    s.try_unify_type_id_type_id_bool_bool_literal_properties(sub_ty, super_ty, false, false, None);
    s.errors
  }

  pub fn can_unify_type_pack_id_type_pack_id_bool(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    is_function_call: bool,
  ) -> ErrorVec {
    let mut child = self.unifier_make_child_unifier();
    child.try_unify_type_pack_id_type_pack_id_bool(sub_tp, super_tp, is_function_call);
    child.errors
  }
}

impl Unifier {
  pub fn check_child_unifier_type_mismatch_error_vec_string_type_id_type_id(
    &mut self,
    inner_errors: &ErrorVec,
    prop: &str,
    wanted_type: TypeId,
    given_type: TypeId,
  ) {
    if let Some(e) = has_unification_too_complex(inner_errors) {
      self.report_error_type_error(e);
    } else if !inner_errors.is_empty() {
      // 手抄 TypeMismatch 构造收敛到 ext 骨架单点（context 仍先于 error 求值）
      self.unifier_report_type_mismatch_ext(
        wanted_type,
        given_type,
        format!("Property '{}' is not compatible.", prop),
        Some(inner_errors[0].clone()),
      );
    }
  }
}

impl Unifier {
  /// `arena` 为 `Handle` 句柄，其目标存活与独占别名契约见 `arena_handle` 模块头。
  pub(crate) fn unifier_combine_logs_into_union(
    &mut self,
    logs: Vec<TxnLog>,
    arena: Handle<TypeArena>,
  ) -> TxnLog {
    let mut result = TxnLog::new();
    for log in logs {
      // Safety: `arena` 沿用本方法文档的句柄契约（目标存活、借用期内无并存可变
      // 别名），与 concat_as_union 的 # Safety 前置条件一致。
      unsafe { result.concat_as_union(log, arena) };
    }
    result
  }
}

impl Unifier {
  pub fn ice_string_location(&mut self, message: &str, location: &Location) {
    self.ice_string(message);
    let _ = location;
  }

  pub fn ice_string(&mut self, message: &str) {
    self.ice_string_location(message, &Location::default());
  }
}

impl Unifier {
  pub fn unifier_make_child_unifier(&mut self) -> Box<Unifier> {
    Box::new(Unifier {
      types: self.types,
      builtin_types: self.builtin_types,
      normalizer: self.normalizer,
      scope: self.scope,
      // 子日志：pending 沿父链上溯、seen 栈与父日志共享同一条
      // （C++ `TxnLog log{&parent.log}`；句柄化后无裸指针接线）。
      log: TxnLog::child_of(&self.log),
      failure: false,
      errors: Vec::new(),
      location: self.location,
      variance: self.variance,
      normalize: self.normalize,
      check_inhabited: self.check_inhabited,
      ctx: self.ctx,
      shared_state: self.shared_state,
      blocked_types: Vec::new(),
      blocked_type_packs: Vec::new(),
      first_pack_error_pos: None,
    })
  }
}

impl Unifier {
  pub fn unifier_mismatch_context(&mut self) -> Context {
    match self.variance {
      Variance::Covariant => Context::CovariantContext,
      Variance::Invariant => Context::InvariantContext,
    }
  }
}

impl Unifier {
  pub fn report_error_location_type_error_data(&mut self, location: Location, data: TypeErrorData) {
    let err = TypeError::type_error_location_type_error_data(location, data);
    self.errors.push(err);
    self.failure = true;
  }

  pub fn report_error_type_error(&mut self, err: TypeError) {
    self.errors.push(err);
    self.failure = true;
  }
}

impl Unifier {
  /// `tryUnify*` 各族「wanted/given 成对报 TypeMismatch（现取 mismatch context）」
  /// 骨架单点，带 reason/error 参数的 ext 版。收口前该块在
  /// `unifier_try_unify_normalized_types.rs` 以 `report_normalized_mismatch` 手抄，
  /// 本函数展开与收口前逐字段等价（`unifier_mismatch_context` 先于 `report_error`
  /// 求值的顺序亦保持；`error.map(Arc::new)` 同款）。
  pub(crate) fn unifier_report_type_mismatch_ext(
    &mut self,
    wanted_type: TypeId,
    given_type: TypeId,
    reason: String,
    error: Option<TypeError>,
  ) {
    let context = self.unifier_mismatch_context();
    self.report_error_location_type_error_data(
      self.location,
      TypeErrorData::TypeMismatch(TypeMismatch {
        wanted_type,
        given_type,
        reason,
        error: error.map(Arc::new),
        context,
      }),
    );
  }

  /// `tryUnify*` 各族「wanted/given 直接成对报 TypeMismatch（空 reason、无 error、
  /// 现取 mismatch context）」骨架单点，委托 ext 版填默认值。
  pub(crate) fn unifier_report_type_mismatch(&mut self, wanted_type: TypeId, given_type: TypeId) {
    self.unifier_report_type_mismatch_ext(wanted_type, given_type, String::new(), None)
  }
}

impl Unifier {
  pub fn unifier_try_unify_negations(&mut self, sub_ty: TypeId, super_ty: TypeId) {
    if get_type::get::<NegationType>(sub_ty).is_none()
      && get_type::get::<NegationType>(super_ty).is_none()
    {
      self.ice_string("tryUnifyNegations super_ty or sub_ty must be a negation type");
    }

    // 归一化过于复杂时与 C++ 一致报错返回
    let (sub_norm, super_norm) = (
      self.normalizer_mut().try_normalize(sub_ty),
      self.normalizer_mut().try_normalize(super_ty),
    );
    let (Some(sub_norm), Some(super_norm)) = (sub_norm, super_norm) else {
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
      );
      return;
    };

    let mut state = self.unifier_make_child_unifier();
    state.unifier_try_unify_normalized_types(
      sub_ty,
      super_ty,
      &sub_norm,
      &super_norm,
      String::new(),
      None,
    );
    if state.errors.is_empty() {
      self.unifier_report_type_mismatch(super_ty, sub_ty);
    }
  }
}

impl Unifier {
  pub fn unifier_try_unify_primitives(&mut self, sub_ty: TypeId, super_ty: TypeId) {
    // C++ Unifier.cpp:1635 tryUnifyPrimitives
    let (Some(super_prim), Some(sub_prim)) = (
      get_type::get::<PrimitiveType>(super_ty),
      get_type::get::<PrimitiveType>(sub_ty),
    ) else {
      self.ice_string("passed non primitive types to unifyPrimitives");
      return;
    };

    if super_prim.r#type != sub_prim.r#type {
      self.unifier_report_type_mismatch(super_ty, sub_ty);
    }
  }
}

impl Unifier {
  pub fn unifier_try_unify_singletons(&mut self, sub_ty: TypeId, super_ty: TypeId) {
    // C++ Unifier.cpp:1646 tryUnifySingletons
    let super_prim = get_type::get::<PrimitiveType>(super_ty);
    let super_singleton = get_type::get::<SingletonType>(super_ty);
    let sub_singleton = get_type::get::<SingletonType>(sub_ty);

    let Some(sub_singleton) = sub_singleton else {
      self.ice_string("passed non singleton/primitive types to unifySingletons");
      return;
    };
    if super_prim.is_none() && super_singleton.is_none() {
      self.ice_string("passed non singleton/primitive types to unifySingletons");
      return;
    }

    // 同一 singleton，直接通过
    if let Some(super_singleton) = super_singleton
      && *super_singleton == *sub_singleton
    {
      return;
    }

    // 协变时：boolean 基类型接受 boolean singleton，string 基类型接受 string singleton
    if let Some(super_prim) = super_prim {
      let covariant = self.variance == Variance::Covariant;
      let boolean_ok = super_prim.r#type == PrimitiveType::BOOLEAN
        && get_singleton_type::<BooleanSingleton>(sub_singleton).is_some();
      let string_ok = super_prim.r#type == PrimitiveType::STRING
        && get_singleton_type::<StringSingleton>(sub_singleton).is_some();

      if covariant && (boolean_ok || string_ok) {
        return;
      }
    }

    self.unifier_report_type_mismatch(super_ty, sub_ty);
  }
}

impl Unifier {
  /// Rust 形态（§2）：C++ `const IntersectionType* uv` → `&'static IntersectionType`，
  /// 判空与解引用收口在调用方的 `alias_opt` 门面，本方法无裸指针。
  pub fn unifier_try_unify_type_with_intersection(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    uv: &'static IntersectionType,
  ) {
    let mut unification_too_complex: Option<TypeError> = None;
    let mut first_failed_option: Option<TypeError> = None;

    for ty in uv.parts.iter().copied() {
      let mut inner_state = self.unifier_make_child_unifier();
      inner_state
        .try_unify_type_id_type_id_bool_bool_literal_properties(sub_ty, ty, false, true, None);

      if let Some(e) = has_unification_too_complex(&inner_state.errors) {
        unification_too_complex = Some(e);
      } else if !inner_state.errors.is_empty() && first_failed_option.is_none() {
        first_failed_option = inner_state.errors.first().cloned();
      }

      self.log.concat(inner_state.log);
      self.failure |= inner_state.failure;
    }

    if let Some(e) = unification_too_complex {
      self.report_error_type_error(e);
    } else if let Some(first) = first_failed_option {
      self.unifier_report_type_mismatch_ext(
        super_ty,
        sub_ty,
        String::from("Not all intersection parts are compatible."),
        Some(first),
      );
    }
  }
}
