//! `type_checker` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{sync::Arc, vec::Vec};
use core::{
  iter::repeat_with,
  ptr::{null, null_mut},
};

use ulua_ast::{
  enums::{ast_expr_ref::AstExprRef, mode::Mode},
  records::{
    ast_expr::AstExpr, ast_stat_block::AstStatBlock, ast_type::AstType, location::Location,
  },
};
use ulua_common::{fflag, fint, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{
    control_flow::ControlFlow, polarity::Polarity, table_state::TableState,
    value_context::ValueContext,
  },
  functions::{
    as_mutable_type::as_mutable_type_id, begin_type::begin_union_type, begin_type_pack::begin,
    contains_never::contains_never as containsNever,
    contains_parse_error_name::contains_parse_error_name,
    diagnose_missing_table_key::diagnose_missing_table_key, end_type_pack::end_type_pack_id,
    filter_map::filter_map as filter_map_type_id, find_metatable_entry::find_metatable_entry,
    find_table_property_respecting_meta_type_utils::find_table_property_respecting_meta,
    follow_type, follow_type_pack, fresh_index::fresh_index, get_mutable_type_pack, get_type,
    get_type_pack::get, is_generic::is_generic, is_prim::is_nil, maybe_generic::maybe_generic,
    merge::merge, quantify::quantify, reduce_union::reduce_union, shared_mut::shared_mut,
    try_strip_union_from_nil::try_strip_union_from_nil,
  },
  records::{
    anyification::Anyification,
    arena_handle::{Handle, alias},
    code_too_complex::CodeTooComplex,
    count_mismatch::CountMismatchContext,
    free_type::FreeType,
    free_type_pack::FreeTypePack,
    function_type::FunctionType,
    generic_type::GenericType,
    never_type::NeverType,
    normalization_too_complex::NormalizationTooComplex,
    optional_value_access::OptionalValueAccess,
    recursion_counter::RecursionCounter,
    scope::Scope,
    scope_registry::register_scope,
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    table_type::TableType,
    txn_log::TxnLog,
    r#type::Type,
    type_checker::TypeChecker,
    type_error::TypeError,
    type_level::TypeLevel,
    type_pack,
    type_pack::TypePack,
    type_pack_var::TypePackVar,
    unification_too_complex::UnificationTooComplex,
    unifier::Unifier,
    union_type::UnionType,
    unknown_property::UnknownProperty,
    variadic_type_pack::VariadicTypePack,
    with_predicate::WithPredicate,
  },
  type_aliases::{
    error_vec::ErrorVec, module_ptr_module::ModulePtr, name_type::Name,
    refinement_map::RefinementMap, scope_ptr_type::ScopePtr, singleton_variant::SingletonVariant,
    type_error_data::TypeErrorData, type_id::TypeId, type_id_predicate::TypeIdPredicate,
    type_pack_id::TypePackId, type_variant::TypeVariant,
  },
};

// Source: `Analysis/src/TypeInfer.cpp:5585-5588` (hand-ported)
// C++ `TypeId TypeChecker::addTV(Type&& tv) { return currentModule->internalTypes.addType(std::move(tv)); }`.

impl TypeChecker {
  pub fn add_tv(&mut self, tv: Type) -> TypeId {
    // currentModule->internalTypes.addType(std::move(tv))
    {
      let module = shared_mut(self.expect_current_module());
      module.internal_types.add_tv(tv)
    }
  }
}

impl TypeChecker {
  pub fn add_type<T>(&mut self, tv: &T) -> TypeId
  where
    T: Clone + Into<Type> + 'static,
  {
    {
      let module = shared_mut(self.expect_current_module());
      module.internal_types.add_type(tv.clone())
    }
  }
}

impl TypeChecker {
  /// C++ `TypePackId TypeChecker::addTypePack(TypePackVar&& tv)` (TypeInfer.cpp:5590):
  /// `return currentModule->internalTypes.addTypePack(std::move(tv));`
  pub fn add_type_pack_type_pack_var(&mut self, tp: TypePackVar) -> TypePackId {
    {
      (shared_mut(self.expect_current_module()))
        .internal_types
        .add_type_pack_type_pack_var(tp)
    }
  }

  /// C++ `TypePackId TypeChecker::addTypePack(TypePack&& tp)` (TypeInfer.cpp:5595):
  /// `return addTypePack(TypePackVar(std::move(tp)));`
  pub fn add_type_pack_type_pack(&mut self, tp: TypePack) -> TypePackId {
    self.add_type_pack_type_pack_var(TypePackVar::from(tp))
  }

  /// C++ `TypePackId TypeChecker::addTypePack(const std::vector<TypeId>& ty, std::optional<TypePackId> tail)`
  /// (TypeInfer.cpp:5605): `return addTypePack(TypePackVar(TypePack{ty, tail}));`
  pub fn add_type_pack_vector_type_id_optional_type_pack_id(
    &mut self,
    ty: &[TypeId],
    tail: Option<TypePackId>,
  ) -> TypePackId {
    self.add_type_pack_type_pack_var(TypePackVar::from(TypePack::new(ty.to_vec(), tail)))
  }

  /// C++ `TypePackId TypeChecker::addTypePack(std::initializer_list<TypeId>&& ty)` (TypeInfer.cpp:5610):
  /// `return addTypePack(TypePackVar(TypePack{std::vector<TypeId>(begin(ty), end(ty)), std::nullopt}));`
  pub fn add_type_pack_initializer_list_type_id(&mut self, ty: &[TypeId]) -> TypePackId {
    self.add_type_pack_type_pack_var(TypePackVar::from(TypePack::from_vec(ty.to_vec())))
  }
}

impl TypeChecker {
  pub fn any_if_nonstrict(&mut self, ty: TypeId) -> TypeId {
    if self.is_nonstrict_mode() {
      self.any_type
    } else {
      ty
    }
  }
}

impl TypeChecker {
  pub(crate) fn anyify_module_return_type_pack_generics(&mut self, tp: TypePackId) -> TypePackId {
    let tp = follow_type_pack::follow(tp);

    if let Some(vtp) = get::<VariadicTypePack>(tp) {
      let ty = follow_type::follow(vtp.ty);
      return if get_type::get::<GenericType>(ty).is_some() {
        self.any_type_pack
      } else {
        tp
      };
    }

    if get::<TypePack>(tp).is_none() {
      return tp;
    }

    let mut result_types = Vec::new();
    let mut result_tail = None;

    let mut it = begin(tp);
    let end_it = end_type_pack_id(tp);

    while it != end_it {
      // SAFETY: iterator 解引用指向 arena 内类型。
      let ty = follow_type::follow(*it.current());
      result_types.push(if get_type::get::<GenericType>(ty).is_some() {
        self.any_type
      } else {
        ty
      });
      it.advance();
    }

    if let Some(tail) = it.tail() {
      result_tail = Some(self.anyify_module_return_type_pack_generics(tail));
    }

    self.add_type_pack_vector_type_id_optional_type_pack_id(&result_types, result_tail)
  }
}

impl TypeChecker {
  pub fn anyify_type_id_location(&mut self, ty: TypeId, location: Location) -> TypeId {
    let arena = Handle::from_mut(&mut shared_mut(self.expect_current_module()).internal_types);
    let mut anyification =
      Anyification::new(arena, self.builtin_types, self.any_type, self.any_type_pack);
    let any = anyification.base.substitute_type_id(ty);
    if anyification.normalization_too_complex {
      self.report_error_location_type_error_data(&location, NormalizationTooComplex.into());
    }
    if let Some(result) = any {
      result
    } else {
      self.report_error_location_type_error_data(&location, UnificationTooComplex.into());
      self.error_recovery_type_type_id(self.any_type)
    }
  }

  pub fn anyify_type_pack_id_location(&mut self, ty: TypePackId, location: Location) -> TypePackId {
    let arena = Handle::from_mut(&mut shared_mut(self.expect_current_module()).internal_types);
    let mut anyification =
      Anyification::new(arena, self.builtin_types, self.any_type, self.any_type_pack);
    let any = anyification.base.substitute_type_pack_id(ty);
    if let Some(any) = any {
      any
    } else {
      self.report_error_location_type_error_data(&location, UnificationTooComplex.into());
      self.error_recovery_type_pack_type_pack_id(self.any_type_pack)
    }
  }
}

impl TypeChecker {
  pub fn can_unify_type_infer(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    scope: &ScopePtr,
    location: &Location,
  ) -> ErrorVec {
    let mut state = self.mk_unifier(scope, location);
    state.can_unify_type_id_type_id(sub_ty, super_ty)
  }

  pub fn can_unify_type_id_type_id_scope_ptr_location(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    scope: &ScopePtr,
    location: &Location,
  ) -> ErrorVec {
    self.can_unify_type_infer(sub_ty, super_ty, scope, location)
  }
}

impl TypeChecker {
  pub fn check_block(&mut self, scope: &ScopePtr, block: &AstStatBlock) -> ControlFlow {
    let _rc = RecursionCounter::recursion_counter_i32(&mut self.check_recursion_count);
    let limit = fint::LuauCheckRecursionLimit.get();
    if limit > 0 && self.check_recursion_count >= limit {
      self.report_error_code_too_complex(&block.base.base.location);
      return ControlFlow::None;
    }

    self.check_block_without_recursion_check(scope, block)
  }
}

impl TypeChecker {
  pub fn check_expr_pack(&mut self, scope: &ScopePtr, expr: &AstExpr) -> WithPredicate<TypePackId> {
    let result = self.check_expr_pack_helper_scope_ptr_ast_expr(scope, expr);
    if containsNever(result.r#type) {
      return WithPredicate::with_predicate_t_predicate_vec(
        self.uninhabitable_type_pack,
        Default::default(),
      );
    }
    result
  }
}

impl TypeChecker {
  // cpp TypeInfer.cpp:3476 checkLValueBinding 的多路分发
  pub fn check_l_value(&mut self, scope: &ScopePtr, expr: &AstExpr, ctx: ValueContext) -> TypeId {
    match expr.as_expr_ref() {
      AstExprRef::Local(local) => self.check_l_value_binding_scope_ptr_ast_expr_local(scope, local),
      AstExprRef::Global(global) => {
        self.check_l_value_binding_scope_ptr_ast_expr_global(scope, global)
      }
      AstExprRef::IndexName(index_name) => self
        .check_l_value_binding_scope_ptr_ast_expr_index_name_value_context(scope, index_name, ctx),
      AstExprRef::IndexExpr(index_expr) => self
        .check_l_value_binding_scope_ptr_ast_expr_index_expr_value_context(scope, index_expr, ctx),
      AstExprRef::Error(error) => {
        for sub_expr in error.expressions.iter_nodes() {
          self.check_expr(scope, sub_expr, None, false);
        }
        self.error_recovery_type_scope_ptr(scope)
      }
      _ => {
        self.ice_string_location("Unexpected AST node in checkLValue", &expr.base.location);
        self.error_recovery_type_scope_ptr(scope)
      }
    }
  }
}

impl TypeChecker {
  pub fn child_function_scope(
    &mut self,
    parent: &ScopePtr,
    location: &Location,
    sub_level: i32,
  ) -> ScopePtr {
    let mut scope_value = Scope::new(parent, sub_level);
    scope_value.location = *location;
    scope_value.return_type = parent.return_type;

    // 公共尾段（注册/挂父/登记 module.scopes）见 register_child_scope。
    self.register_child_scope(parent, scope_value, location)
  }
}

impl TypeChecker {
  pub fn child_scope(&mut self, parent: &ScopePtr, location: &Location) -> ScopePtr {
    let mut scope_value = Scope::new(parent, 0);
    scope_value.level = parent.level;
    scope_value.vararg_pack = parent.vararg_pack;
    scope_value.location = *location;
    scope_value.return_type = parent.return_type;

    self.register_child_scope(parent, scope_value, location)
  }

  /// 两处子作用域构造（`child_scope`/`child_function_scope`）的公共尾段：
  /// Arc 装箱 → 注册发放句柄 → 挂父 `children` → 登记进 `module.scopes`
  /// （cpp TypeChecker.cpp 两函数同型尾段）。差异只在前置的 Scope 字段
  /// 初始化，由调用方完成后整体传入。
  pub(crate) fn register_child_scope(
    &mut self,
    parent: &ScopePtr,
    scope_value: Scope,
    location: &Location,
  ) -> ScopePtr {
    let scope = Arc::new(scope_value);
    // 注册发放本 scope 的句柄，父 children 只存句柄（裸地址仅经注册点入系统）。
    let scope_id = register_scope(&scope);

    {
      let parent_mut = shared_mut(parent);
      parent_mut.children.push(scope_id);

      let module = shared_mut(self.expect_current_module());
      module.scopes.push((*location, scope.clone()));
    }

    scope
  }
}

/// `current_module` 非空不变式的单点表述（原 40 余处逐字重复的 expect 文案
/// 收敛于此）：cpp `TypeChecker::currentModule` 是直接持有的模块引用、不存在
/// 空态；Rust 以 `Option<ModulePtr>` 建模，`check_without_recursion_check`
/// 入口置入 `Some`、末尾才 `take()`，整个 check 调用树内恒为 `Some`。
pub(crate) const CURRENT_MODULE_INVARIANT: &str = "current_module 由 check_without_recursion_check 入口置入 Some、末尾才 take()，check 调用树内恒为 Some";
impl TypeChecker {
  /// `current_module` 的非空读取收口：等价 cpp 直接解引用 `currentModule`。
  pub fn expect_current_module(&self) -> &ModulePtr {
    self
      .current_module
      .as_ref()
      .expect(CURRENT_MODULE_INVARIANT)
  }
}

impl TypeChecker {
  pub fn error_recovery_type_pack_scope_ptr(&mut self, _scope: ScopePtr) -> TypePackId {
    // builtin_types 是 Handle（NonNull 编码非空，构造期从 Frontend 持有的全局
    // BuiltinTypes 接线、比 self 长寿）；get() 只读物化借用读 Copy 的
    // error_type_pack 常量字段，契约收拢于 arena_handle 模块。
    self.builtin_types.get().error_type_pack
  }

  pub fn error_recovery_type_pack_type_pack_id(&mut self, guess: TypePackId) -> TypePackId {
    // builtin_types 句柄目标会话期存活（见 arena_handle 契约）；
    // error_recovery_type_pack 取 &self（安全方法），get() 物化的共享借用只读、
    // 无在册可变别名；guess 由调用方传入的存活 TypePackId，本函数不解引用它。
    self.builtin_types.get().error_recovery_type_pack(guess)
  }
}

impl TypeChecker {
  pub fn error_recovery_type_scope_ptr(&mut self, _scope: &ScopePtr) -> TypeId {
    // builtin_types 是 Handle（NonNull 编码非空，构造期由 C++ NotNull 形参接线的
    // 进程/模块级 BuiltinTypes，比 type checker 长寿）；get() 只读物化借用读
    // error_type 的 TypeId 值，无写入、无别名冲突。
    self.builtin_types.get().error_type
  }

  pub fn error_recovery_type_type_id(&mut self, guess: TypeId) -> TypeId {
    // builtin_types 句柄目标非空且长寿（见 arena_handle 契约）；
    // error_recovery_type 走 &self 只读路径，get() 物化的借用唯一，
    // guess 仅透传给该 &self 方法，本次调用内借用唯一。
    self.builtin_types.get().error_recovery_type(guess)
  }
}

impl TypeChecker {
  /// C++ `std::pair<std::optional<TypeId>, bool> TypeChecker::filterMap(TypeId, TypeIdPredicate)`
  /// (`Analysis/src/TypeInfer.cpp:5567-5571`)。
  pub fn filter_map<P: TypeIdPredicate>(
    &mut self,
    r#type: TypeId,
    predicate: &mut P,
  ) -> (Option<TypeId>, bool) {
    let ty = self
      .filter_map_impl(r#type, predicate)
      .unwrap_or(self.never_type);
    // C++ 用 `get<NeverType>(ty)` 做类型判断（跟随别名），不能退化为指针比较。
    let ty_is_never = get_type::get::<NeverType>(ty).is_some();
    (Some(ty), !ty_is_never)
  }
}

impl TypeChecker {
  /// C++ `std::optional<TypeId> TypeChecker::filterMapImpl(TypeId, TypeIdPredicate)`
  /// (`Analysis/src/TypeInfer.cpp:5559-5565`)。predicate 以泛型传入，
  /// 上游 `TypeIdPredicate` 是 `std::function`，这里无需 dyn/Box。
  pub fn filter_map_impl<P: TypeIdPredicate>(
    &mut self,
    r#type: TypeId,
    predicate: &mut P,
  ) -> Option<TypeId> {
    let types = filter_map_type_id(r#type, self, predicate);
    if types.is_empty() {
      return None;
    }

    Some(if types.len() == 1 {
      types[0]
    } else {
      self.add_type(&UnionType { options: types })
    })
  }
}

impl TypeChecker {
  pub(crate) fn find_metatable_entry(
    &mut self,
    ty: TypeId,
    entry: &str,
    location: &Location,
    add_errors: bool,
  ) -> Option<TypeId> {
    let mut errors: ErrorVec = ErrorVec::new();
    let result = find_metatable_entry(self.builtin_types, &mut errors, ty, entry, *location);
    if add_errors {
      self.report_errors(&errors);
    }
    result
  }
}

impl TypeChecker {
  pub fn find_table_property_respecting_meta(
    &mut self,
    lhs_type: TypeId,
    name: Name,
    location: &Location,
    add_errors: bool,
  ) -> Option<TypeId> {
    let mut errors: ErrorVec = ErrorVec::new();
    let result = find_table_property_respecting_meta(
      self.builtin_types,
      &mut errors,
      lhs_type,
      name.as_str(),
      ValueContext::RValue,
      *location,
      false,
    );
    if add_errors {
      self.report_errors(&errors);
    }
    result
  }
}

impl TypeChecker {
  /// C++ `freshType(sharedState, scope, ...)`：只读取 `scope->level`，
  /// 以 `&Scope` 入参避免调用方为传值克隆 `ScopePtr`（Arc 引用计数）。
  pub fn fresh_type_pack_scope_ptr(&mut self, scope: &Scope) -> TypePackId {
    self.fresh_type_pack_type_level(scope.level)
  }

  pub fn fresh_type_pack_type_level(&mut self, level: TypeLevel) -> TypePackId {
    {
      let module = shared_mut(self.expect_current_module());
      module.internal_types.add_type_pack_t(FreeTypePack {
        index: fresh_index(),
        level,
        scope: null_mut(),
        polarity: Polarity::None,
      })
    }
  }
}

impl TypeChecker {
  pub fn fresh_type_scope_ptr(&mut self, scope: ScopePtr) -> TypeId {
    self.fresh_type_type_level(scope.level)
  }

  pub fn fresh_type_type_level(&mut self, level: TypeLevel) -> TypeId {
    {
      let module = shared_mut(self.expect_current_module());
      module
        .internal_types
        .fresh_type_not_null_builtin_types_type_level(self.builtin_types.get(), level)
    }
  }
}

impl TypeChecker {
  pub fn get_index_type_from_type(
    &mut self,
    _scope: ScopePtr,
    _type: TypeId,
    _name: &Name,
    _location: &Location,
    _add_errors: bool,
  ) -> Option<TypeId> {
    let error_count = self.expect_current_module().errors.len();
    let result = self.get_index_type_from_type_impl(_scope, _type, _name, _location, _add_errors);
    if !_add_errors {
      LUAU_ASSERT!(error_count == self.expect_current_module().errors.len());
    }
    result
  }
}

impl TypeChecker {
  pub fn ice_string_location(&mut self, message: &str, location: &Location) {
    self.ice_string(message);
    let _ = location;
  }

  pub fn ice_string(&mut self, message: &str) {
    self.ice_string_location(message, &Location::default());
  }
}

impl TypeChecker {
  pub fn instantiate(
    &mut self,
    scope: &ScopePtr,
    ty: TypeId,
    location: Location,
    log: *const TxnLog,
  ) -> TypeId {
    let ty = follow_type::follow(ty);

    if let Some(ftv) = get_type::get::<FunctionType>(ty)
      && ftv.has_no_free_or_generic_types
    {
      return ty;
    }

    // reusableInstantiation.resetState(log, &currentModule->internalTypes, builtinTypes, scope->level, /*scope*/ nullptr);
    {
      let arena = Handle::from_mut(&mut (shared_mut(self.expect_current_module())).internal_types);
      self
        .reusable_instantiation
        .reset_state(log, arena, self.builtin_types, scope.level, None);
    }

    if let Some(child_limit) = self.instantiation_child_limit {
      self.reusable_instantiation.base.base.child_limit = child_limit;
    }

    let instantiated = self.reusable_instantiation.substitute_type_id(ty);

    if let Some(instantiated) = instantiated {
      instantiated
    } else {
      self.report_error_location_type_error_data(&location, UnificationTooComplex.into());
      self.error_recovery_type_scope_ptr(scope)
    }
  }
}

impl TypeChecker {
  pub fn is_nonstrict_mode(&self) -> bool {
    let module = self.expect_current_module();
    matches!(module.mode, Mode::Nonstrict | Mode::NoCheck)
  }
}

impl TypeChecker {
  pub fn merge(&mut self, l: &mut RefinementMap, r: &RefinementMap) {
    // 上游 lambda 捕获 `this` 并调用 `addType`（TypeInfer.cpp:5460-5482）。
    // `Luau::merge` 对每个 key 至多调用一次回调（LValue.cpp:91-101），所以
    // `impl FnMut` 直接可变借用 self 就够了，无需伪造 `*mut TypeChecker` 别名。
    merge(l, r, |a, b| {
      // Insertion-order dedup (not HashSet<TypeId>): a pointer-keyed HashSet
      // iterates in per-instance randomized order, which would make the merged
      // union's option order — and every diagnostic derived from it —
      // nondeterministic across runs. Preserving a-then-b order is stable.
      let mut options: Vec<TypeId> = Vec::new();
      let mut push = |t: TypeId| {
        if !options.contains(&t) {
          options.push(t);
        }
      };

      fn flatten_into(push: &mut impl FnMut(TypeId), ty: TypeId) {
        match get_type::get::<UnionType>(follow_type::follow(ty)) {
          // C++ `set.insert(begin(utv), end(utv))`——UnionTypeIterator 展平
          // 嵌套 union 并 follow，裸遍历 options 会漏掉嵌套成员。
          Some(utv) => begin_union_type(utv).for_each(push),
          None => push(ty),
        }
      }
      flatten_into(&mut push, a);
      flatten_into(&mut push, b);

      if options.len() == 1 {
        options[0]
      } else {
        self.add_type(&UnionType { options })
      }
    });
  }
}

impl TypeChecker {
  pub fn pick_types_from_sense(
    &mut self,
    _type: TypeId,
    _sense: bool,
    _empty_set_ty: TypeId,
  ) -> (Option<TypeId>, bool) {
    let mut predicate = self.mk_truthy_predicate(_sense, _empty_set_ty);
    self.filter_map(_type, &mut predicate)
  }
}

impl TypeChecker {
  pub fn prepare_errors_for_display(&mut self, err_vec: &mut ErrorVec) {
    err_vec.retain(|err| !contains_parse_error_name(err));

    for err in err_vec.iter_mut() {
      if let TypeErrorData::UnknownProperty(utk) = err.data.clone() {
        diagnose_missing_table_key(&utk, &mut err.data);
      }
    }
  }
}

impl TypeChecker {
  pub fn quantify(&mut self, scope: &ScopePtr, ty: TypeId, _location: Location) -> TypeId {
    let ty = follow_type::follow(ty);

    let ftv = get_type::get::<FunctionType>(ty);

    if ftv.is_some() {
      quantify(ty, scope.level);
    }

    ty
  }
}

impl TypeChecker {
  pub fn report_error_code_too_complex(&mut self, location: &Location) {
    let error = TypeError::type_error_location_type_error_data(*location, CodeTooComplex.into());
    self.report_error_type_error(&error);
  }
}

impl TypeChecker {
  pub fn report_error_type_error(&mut self, error: &TypeError) {
    let module = self.expect_current_module();

    if module.mode == Mode::NoCheck {
      return;
    }

    {
      let module = shared_mut(module);
      module.errors.push(error.clone());
      // Safety: 紧邻上方 push 之后取尾，必命中（cpp errors.back() 同位）。
      module
        .errors
        .last_mut()
        .expect("紧邻 push 之后取尾，必命中")
        .module_name = module.name.clone();
    }
  }

  pub fn report_error_location_type_error_data(
    &mut self,
    _location: &Location,
    _error_data: TypeErrorData,
  ) {
    let error = TypeError::type_error_location_type_error_data(*_location, _error_data);
    self.report_error_type_error(&error);
  }

  /// `UnknownProperty` 报错单点收口：原先 get_index_type_from_type_impl /
  /// check_l_value_binding_type_infer 里 8 处同构报错块共用此入口。
  /// 行为等价于内联的
  /// `report_error_location_type_error_data(location, UnknownProperty { table, key })`。
  pub(crate) fn report_unknown_property(&mut self, location: &Location, table: TypeId, key: &Name) {
    self.report_error_location_type_error_data(
      location,
      TypeErrorData::UnknownProperty(UnknownProperty {
        table,
        key: key.clone(),
      }),
    );
  }
}

impl TypeChecker {
  pub fn report_errors(&mut self, errors: &ErrorVec) {
    for err in errors {
      self.report_error_type_error(err);
    }
  }
}

impl TypeChecker {
  pub fn resolve_type(&mut self, scope: ScopePtr, annotation: &AstType) -> TypeId {
    let ty = self.resolve_type_worker(scope, annotation);

    {
      let module = shared_mut(self.expect_current_module());
      *module
        .ast_resolved_types
        .get_or_insert(annotation as *const AstType) = ty;
    }

    ty
  }
}

impl TypeChecker {
  pub fn singleton_type_bool(&mut self, value: bool) -> TypeId {
    // builtin_types 为 Handle（NonNull 编码非空）持有的会话级单例，get() 物化
    // 只读借用取其 Copy 的 true/false 常量 TypeId。
    if value {
      self.builtin_types.get().true_type
    } else {
      self.builtin_types.get().false_type
    }
  }

  pub fn singleton_type_string(&mut self, value: String) -> TypeId {
    let singleton = SingletonType::new(SingletonVariant::V1(StringSingleton::new(value)));
    let ty = Type::new(TypeVariant::Singleton(singleton));
    self.add_type(&ty)
  }
}

impl TypeChecker {
  pub fn strip_from_nil_and_report(&mut self, ty: TypeId, location: &Location) -> TypeId {
    let ty = follow_type::follow(ty);

    // C++ `std::any_of(begin(utv), end(utv), isNil)` — UnionTypeIterator 防环展平并 follow。
    if let Some(utv) = get_type::get::<UnionType>(ty)
      && !begin_union_type(utv).any(is_nil)
    {
      return ty;
    }

    if let Some(stripped_union) = self.try_strip_union_from_nil(ty) {
      self.report_error_location_type_error_data(
        location,
        TypeErrorData::OptionalValueAccess(OptionalValueAccess { optional: ty }),
      );
      return follow_type::follow(stripped_union);
    }

    ty
  }
}

impl TypeChecker {
  pub fn tablify(&mut self, ty: TypeId) {
    let ty = follow_type::follow(ty);

    if let Some(free) = get_type::get::<FreeType>(ty) {
      alias(as_mutable_type_id(ty)).ty =
        TypeVariant::Table(TableType::table_type_table_state_type_level_scope(
          TableState::Free,
          free.level,
          null_mut(),
        ));
    }
  }
}

// `TypeChecker::tryStripUnionFromNil` 的核心遍历已单源化于
// [`crate::functions::try_strip_union_from_nil`]（cpp TypeUtils.cpp 同名
// static 函数），此处仅做 arena 接线。

impl TypeChecker {
  pub fn try_strip_union_from_nil(&mut self, ty: TypeId) -> Option<TypeId> {
    // SAFETY: current_module 在类型检查期间独占（与 self.add_type 同一降级
    // 路径：shared_mut 短时重建 &mut，借用止于本次调用，对应 C++ 直接持有
    // module->internal_types）。
    {
      let module = shared_mut(self.expect_current_module());
      try_strip_union_from_nil(&mut module.internal_types, ty)
    }
  }
}

impl TypeChecker {
  pub fn un_type_pack(
    &mut self,
    scope: &ScopePtr,
    tp: TypePackId,
    expected_length: usize,
    location: &Location,
  ) -> Vec<TypeId> {
    let expected_type_pack = self.add_type_pack_type_pack(TypePack::empty());
    // 刚以 TypePack 变体分配，下转必然成功（C++ LUAU_ASSERT(expectedPack)）。
    let expected_pack =
      get_mutable_type_pack::get_mutable::<type_pack::TypePack>(expected_type_pack)
        .expect("刚以 TypePack 变体分配，下转必命中（C++ LUAU_ASSERT(expectedPack)）");
    expected_pack
      .head
      .extend(repeat_with(|| self.fresh_type_scope_ptr(scope.clone())).take(expected_length));
    let old_errors_size = self.expect_current_module().errors.len();
    self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
      tp,
      expected_type_pack,
      scope,
      location,
      CountMismatchContext::Arg,
    );
    (shared_mut(self.expect_current_module()))
      .errors
      .truncate(old_errors_size);
    let mut result = expected_pack.head.clone();
    for ty in &mut result {
      *ty = follow_type::follow(*ty);
    }
    result
  }
}

impl TypeChecker {
  pub fn unify_with_instantiation_if_needed_type_id_type_id_scope_ptr_unifier(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    scope: ScopePtr,
    state: &mut Unifier,
  ) {
    LUAU_ASSERT!(!fflag::LuauInstantiateInSubtyping.get());

    if !maybe_generic(sub_ty) {
      state.try_unify_type_id_type_id_bool_bool_literal_properties(
        sub_ty, super_ty, false, false, None,
      );
    } else if !maybe_generic(super_ty) && is_generic(sub_ty) {
      let instantiated = self.instantiate(&scope, sub_ty, state.location, null());
      state.try_unify_type_id_type_id_bool_bool_literal_properties(
        instantiated,
        super_ty,
        false,
        false,
        None,
      );
    } else {
      let mut child = state.unifier_make_child_unifier();
      child.try_unify_type_id_type_id_bool_bool_literal_properties(
        sub_ty, super_ty, false, false, None,
      );

      if !child.errors.is_empty() {
        let instantiated = self.instantiate(&scope, sub_ty, state.location, &child.log as *const _);

        if sub_ty == instantiated {
          state.log.concat(child.log);
          state.errors.append(&mut child.errors);
        } else {
          state.try_unify_type_id_type_id_bool_bool_literal_properties(
            instantiated,
            super_ty,
            false,
            false,
            None,
          );
        }
      } else {
        state.log.concat(child.log);
      }
    }
  }
}

impl TypeChecker {
  pub fn union_of_types(
    &mut self,
    a: TypeId,
    b: TypeId,
    scope: &ScopePtr,
    location: &Location,
    unify_free_types: bool,
  ) -> TypeId {
    let a = follow_type::follow(a);
    let b = follow_type::follow(b);

    if unify_free_types {
      let is_a_free = get_type::get::<FreeType>(a).is_some();
      let is_b_free = get_type::get::<FreeType>(b).is_some();

      if is_a_free || is_b_free {
        if self.unify_type_id_type_id_scope_ptr_location(b, a, scope, location) {
          return a;
        }

        return self.error_recovery_type_type_id(self.any_type);
      }
    }

    if a == b {
      return a;
    }

    let types = reduce_union(&[a, b]);
    if types.is_empty() {
      return self.never_type;
    }

    if types.len() == 1 {
      return types[0];
    }

    self.add_type(&UnionType { options: types })
  }
}
