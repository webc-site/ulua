//! `type_checker_2` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::{cmp, mem::take};

use ulua_ast::{
  enums::ast_stat_ref::AstStatRef,
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_local::AstExprLocal, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_for_in::AstStatForIn, ast_type_pack::AstTypePack, location::Location,
  },
  rtti::{ast_node_try_as, ast_node_try_as_ptr},
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{normalization_result::NormalizationResult, value::Value, value_context::ValueContext},
  functions::{
    begin_type::begin_union_type,
    diagnose_missing_table_key::diagnose_missing_table_key,
    follow_type, follow_type_pack, get_type,
    is_prim::is_nil,
    push_module_scope::push_module_scope,
    should_suppress_errors_type_utils::{
      should_suppress_errors, should_suppress_errors_not_null_normalizer_type_pack_id,
    },
    try_strip_union_from_nil::try_strip_union_from_nil,
  },
  records::{
    arena_handle::{Handle, alias_ref},
    function_type::FunctionType,
    generic_type_visitor::GenericTypeVisitorTrait,
    internal_type_function_finder::InternalTypeFunctionFinder,
    normalization_too_complex::NormalizationTooComplex,
    optional_value_access::OptionalValueAccess,
    pack_where_clause_needed::PackWhereClauseNeeded,
    scope::Scope,
    scope_registry::resolve_scope,
    stack_pusher::StackPusher,
    symbol::Symbol,
    type_checker_2::TypeChecker2,
    type_error::TypeError,
    type_function_instance_type::TypeFunctionInstanceType,
    union_type::UnionType,
    unknown_property::UnknownProperty,
    where_clause_needed::WhereClauseNeeded,
  },
  type_aliases::{
    error_vec::ErrorVec,
    name_type::Name,
    type_error_data::{TypeErrorData, TypeErrorDataMember},
    type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

impl TypeChecker2 {
  pub fn check_for_internal_type_function(&mut self, ty: TypeId, location: Location) {
    let mut finder = InternalTypeFunctionFinder::new(&mut self.function_decl_stack);
    finder.traverse_type_id(ty);

    for internal in finder.internal_functions.iter() {
      if self.should_suppress_uninhabited_type_function_error(*internal) {
        continue;
      }

      self.report_error_type_error_data_location(
        WhereClauseNeeded { ty: *internal }.into(),
        &location,
      );
    }

    for internal in finder.internal_pack_functions.iter() {
      self.report_error_type_error_data_location(
        PackWhereClauseNeeded { tp: *internal }.into(),
        &location,
      );
    }
  }
}

// Faithful port of `TypeChecker2::findInnermostScope` (TypeChecker2.cpp:609-629).

impl TypeChecker2 {
  /// Scope* bestScope = module->getModuleScope().get();
  /// 返回 [`Handle<Scope>`]：模块根作用域由 `get_module_scope()` 的 `Arc<Scope>`
  /// 克隆保活，children 经 `ScopeId` 注册表还原（scope_registry 契约），句柄只
  /// 复制非空地址，解引用契约集中在 `arena_handle`。
  pub fn find_innermost_scope(&self, location: Location) -> Handle<Scope> {
    let module_scope = self.module_ref().get_module_scope();
    let mut best_scope: Handle<Scope> = Handle::from_ref(module_scope.as_ref());

    loop {
      let mut did_narrow = false;
      // children 持 ScopeId 句柄（NotNull<Scope*> 的句柄化）：只读经
      // resolve_scope 取子 scope 判 location，命中后换新句柄继续下钻
      // （起点为函数局部 Arc，之后为注册表保活的子 Scope）。
      for &child_id in best_scope.get().children.iter() {
        let Some(child) = resolve_scope(child_id) else {
          continue;
        };
        if child.location.encloses(&location) {
          best_scope = Handle::from_ref(child);
          did_narrow = true;
          break;
        }
      }

      if !(did_narrow && !best_scope.get().children.is_empty()) {
        break;
      }
    }

    best_scope
  }
}

impl TypeChecker2 {
  // cpp TypeChecker2.cpp:1221
  pub fn get_binding_type(&mut self, expr: &AstExpr) -> Option<TypeId> {
    // 栈已句柄化：`s` 为栈顶 `Handle<Scope>`（`?` 已排除空栈），目标由
    // push_stack 自 `module.ast_scopes` 登记的存活作用域，`get()` 解引用
    // 契约集中在 `arena_handle`；`lookup_symbol` 为 `&self` 只读。
    let s = *self.stack.last()?;

    if let Some(local_expr) = ast_node_try_as::<AstExprLocal>(&expr.base) {
      // local 槽已句柄化恒非空；Symbol::from_local 为既有裸指针 API，经 as_ptr 桥接。
      return s
        .get()
        .lookup_symbol(Symbol::from_local(local_expr.local.as_ptr()));
    }

    if let Some(global_expr) = ast_node_try_as::<AstExprGlobal>(&expr.base) {
      return s.get().lookup_symbol(Symbol::from_global(global_expr.name));
    }

    None
  }
}

impl TypeChecker2 {
  /// cpp `Analysis/src/TypeChecker2.cpp:346`
  /// （`Location TypeChecker2::getEndLocation(const AstExprFunction*)`）。
  /// `function` 为 arena 存活节点的共享引用（非空由引用类型承载），仅只读其
  /// `location` 字段。
  pub fn get_end_location(&self, function: &AstExprFunction) -> Location {
    let mut loc = function.base.base.location;

    if loc.begin.line != loc.end.line {
      let mut begin = loc.end;
      begin.column = cmp::max(0, begin.column as i32 - 3) as u32;
      loc = Location::new(begin, loc.end);
    }

    loc
  }
}

impl TypeChecker2 {
  /// cpp `Analysis/src/TypeChecker2.cpp:381`（`bool TypeChecker2::hasBreak(AstStat*)`）。
  pub fn type_checker_2_has_break(&mut self, node: &AstStat) -> bool {
    // cpp TypeChecker2.cpp:381
    match node.as_stat_ref() {
      AstStatRef::Block(block) => block
        .body
        .iter_nodes()
        .any(|stat| self.type_checker_2_has_break(stat.get())),
      AstStatRef::Break(_) => true,
      AstStatRef::If(if_stat) => {
        self.type_checker_2_has_break(&if_stat.thenbody.get().base)
          || if_stat
            .elsebody
            .get()
            .is_some_and(|elsebody| self.type_checker_2_has_break(elsebody))
      }
      _ => false,
    }
  }
}

impl TypeChecker2 {
  /// cpp `Analysis/src/TypeChecker2.cpp:359`（`bool TypeChecker2::isErrorCall(const AstExprCall*)`）。单线程。
  // cpp TypeChecker2.cpp:359
  pub fn is_error_call(&mut self, call: &AstExprCall) -> bool {
    // Safety: parser 保证 AstExprCall.func 子节点指针非空（非 Optional 字段）；
    // ast_node_try_as_ptr 按 RTTI class index 分派——未命中返回 None，命中即动态
    // 类型确为 AstExprGlobal，且 repr(C) 单继承首字段基址重合使 cast 指向完整节点；
    // 节点随 parse 树存活至 check 结束。
    let Some(global) = (unsafe { ast_node_try_as_ptr::<AstExprGlobal>(call.func) }) else {
      return false;
    };
    let name = global.name.as_str_or_empty();

    if name == "error" {
      return true;
    }
    if name == "assert" {
      return call.args.is_empty()
        || call.args.first().is_some_and(|&arg| unsafe {
          ast_node_try_as_ptr::<AstExprConstantBool>(arg).is_some_and(|b| !b.value)
        });
    }

    false
  }
}

impl TypeChecker2 {
  pub(crate) fn is_error_suppressing_location_type_id(
    &mut self,
    loc: Location,
    ty: TypeId,
  ) -> bool {
    match should_suppress_errors(&mut self.normalizer, ty).error_suppression_value() {
      Value::DoNotSuppress => false,
      Value::Suppress => true,
      Value::NormalizationFailed => {
        self.report_error_type_error_data_location(
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
          &loc,
        );
        false
      }
    }
  }

  pub fn is_error_suppressing_location_type_pack_id(
    &mut self,
    loc: Location,
    tp: TypePackId,
  ) -> bool {
    match should_suppress_errors_not_null_normalizer_type_pack_id(&mut self.normalizer, tp)
      .error_suppression_value()
    {
      Value::DoNotSuppress => false,
      Value::Suppress => true,
      Value::NormalizationFailed => {
        self.report_error_type_error_data_location(
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
          &loc,
        );
        false
      }
    }
  }
}

impl TypeChecker2 {
  pub fn lookup_pack(&self, expr: *mut AstExpr) -> TypePackId {
    // If a type isn't in the type graph, it probably means that a recursion limit was exceeded.
    // We'll just return any_type in these cases.  Typechecking against any is very fast and this
    // allows us not to think about this very much in the actual typechecking logic.
    let module = self.module_ref();
    let tp = module.ast_type_packs.find(&(expr as *const AstExpr));
    if let Some(tp) = tp {
      follow_type_pack::follow(*tp)
    } else {
      self.builtin_types_ref().any_type_pack
    }
  }
}

impl TypeChecker2 {
  pub fn lookup_pack_annotation(&self, annotation: *mut AstTypePack) -> Option<TypePackId> {
    let tp = unsafe {
      (*self.module)
        .ast_resolved_type_packs
        .find(&(annotation as *const AstTypePack))
    };
    tp.map(|tp| follow_type_pack::follow(*tp))
  }
}

// Faithful port of `TypeChecker2::lookupType` (TypeChecker2.cpp:522-536).

impl TypeChecker2 {
  pub fn lookup_type(&mut self, expr: &AstExpr) -> TypeId {
    // If a type isn't in the type graph, it probably means that a recursion limit was exceeded.
    // We'll just return any_type in these cases.  Typechecking against any is very fast and this
    // allows us not to think about this very much in the actual typechecking logic.
    let location = expr.base.location;

    // SAFETY: self.module 与类型检查会话同寿（C++ 同契约）。
    let ty = self
      .module_ref()
      .ast_types
      .find(&(expr as *const AstExpr))
      .copied();
    if let Some(ty) = ty {
      return self.check_for_type_function_inhabitance(follow_type::follow(ty), location);
    }

    // SAFETY: 同上。
    let tp = self
      .module_ref()
      .ast_type_packs
      .find(&(expr as *const AstExpr))
      .copied();
    if let Some(tp) = tp {
      let flattened = self.flatten_pack(tp);
      return self.check_for_type_function_inhabitance(flattened, location);
    }

    // SAFETY: builtin_types 由构造方保证有效（C++ 同契约）。
    self.builtin_types_ref().any_type
  }
}

impl TypeChecker2 {
  // C++ `std::optional<StackPusher> TypeChecker2::pushStack(AstNode* node)`
  // (TypeChecker2.cpp:476): if the node has a recorded scope, push it onto the
  // scope stack for the lifetime of the returned guard; otherwise nullopt.
  // 实现收口于 `push_module_scope`（与旧求解器 `push_stack` 共享同一段查表-压栈）。
  pub fn push_stack(&mut self, node: &AstNode) -> Option<StackPusher> {
    push_module_scope(self.module, &mut self.stack, node)
  }
}

impl TypeChecker2 {
  pub fn report_error_type_error_data_location(
    &mut self,
    mut data: TypeErrorData,
    location: &Location,
  ) {
    // if (auto utk = get_if<UnknownProperty>(&data)) diagnoseMissingTableKey(utk, data);
    if let Some(utk) = UnknownProperty::get_if(&data) {
      // C++ holds a pointer into `data` while also mutating `data`;
      // clone the property so the borrow checker is satisfied without
      // changing behaviour (diagnoseMissingTableKey only reads `utk`).
      let utk = utk.clone();
      diagnose_missing_table_key(&utk, &mut data);
    }

    // module->errors.emplace_back(location, module->name, std::move(data));
    // 经 `module_mut` 访问器收口（module 字段裸解引用契约集中在访问器）。
    let module_name = self.module_ref().name.clone();
    self
      .module_mut()
      .errors
      .push(TypeError::type_error_location_module_name_type_error_data(
        *location,
        module_name,
        data,
      ));

    // if (logger) logger->captureTypeCheckError(module->errors.back());
    // `Option<Handle>` 承载原 C++ `DcrLogger*` 判空语义，一一对应；
    // `capture_type_check_error` 只读取刚 push 的栈顶错误。
    if let Some(logger) = self.logger {
      let last = self
        .module_ref()
        .errors
        .last()
        .expect("errors 刚 push 过，last() 必非空");
      logger.get_mut().capture_type_check_error(last);
    }
  }

  pub fn report_error_type_error(&mut self, e: TypeError) {
    self.report_error_type_error_data_location(e.data, &e.location);
  }

  /// `UnknownProperty` 报错单点收口：原先 check_index_type_from_type 读/写两路
  /// 2 处同构报错块共用此入口。仍走 `report_error_type_error_data_location`，
  /// 保留 `diagnose_missing_table_key` 拦截，行为等价于内联写法。
  pub(crate) fn report_unknown_property(&mut self, location: &Location, table: TypeId, key: &Name) {
    self.report_error_type_error_data_location(
      TypeErrorData::UnknownProperty(UnknownProperty {
        table,
        key: key.clone(),
      }),
      location,
    );
  }
}

impl TypeChecker2 {
  pub fn report_errors(&mut self, errors: ErrorVec) {
    for e in errors {
      self.report_error_type_error(e);
    }
  }
}

impl TypeChecker2 {
  pub fn should_suppress_uninhabited_type_function_error(&mut self, ty: TypeId) -> bool {
    let ty = follow_type::follow(ty);
    let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(ty) else {
      return false;
    };

    let function_name = tfit.function().name.as_str();
    let is_numeric = matches!(
      function_name,
      "add" | "sub" | "mul" | "div" | "idiv" | "pow" | "mod"
    );

    if !is_numeric {
      return false;
    }

    for arg in &tfit.type_arguments {
      let arg = follow_type::follow(*arg);
      let Some(normalized) = self.normalizer.try_normalize(arg) else {
        continue;
      };

      if self
        .normalizer
        .is_inhabited_normalized_type(normalized.as_ref())
        == NormalizationResult::False
      {
        return true;
      }
    }

    false
  }
}

// Faithful port of `TypeChecker2::stripFromNilAndReport` (TypeChecker2.cpp:1883-1910).

impl TypeChecker2 {
  pub fn strip_from_nil_and_report(&mut self, ty: TypeId, location: &Location) -> TypeId {
    let ty = follow_type::follow(ty);

    // if (auto utv = get<UnionType>(ty))
    //     if (!std::any_of(begin(utv), end(utv), isNil)) return ty;
    // UnionTypeIterator 防环展平并 follow。
    if let Some(utv) = get_type::get::<UnionType>(ty)
      && !begin_union_type(utv).any(is_nil)
    {
      return ty;
    }

    if let Some(stripped_union) = self.try_strip_union_from_nil(ty) {
      match Value::from(should_suppress_errors(&mut self.normalizer, ty)) {
        Value::Suppress => {}
        Value::NormalizationFailed => {
          self.report_error_type_error_data_location(
            NormalizationTooComplex::default().into(),
            location,
          );
          // [[fallthrough]]
          self.report_error_type_error_data_location(
            OptionalValueAccess { optional: ty }.into(),
            location,
          );
        }
        Value::DoNotSuppress => {
          self.report_error_type_error_data_location(
            OptionalValueAccess { optional: ty }.into(),
            location,
          );
        }
      }

      return follow_type::follow(stripped_union);
    }

    ty
  }
}

impl TypeChecker2 {
  pub fn test_is_subtype_for_in_stat(
    &mut self,
    iter_func: TypeId,
    prospective_func: TypeId,
    for_in_stat: &AstStatForIn,
  ) {
    LUAU_ASSERT!(get_type::get::<FunctionType>(follow_type::follow(iter_func)).is_some());
    LUAU_ASSERT!(get_type::get::<FunctionType>(follow_type::follow(prospective_func)).is_some());

    // for_in_stat.values 至少含一个迭代函数表达式（语法保证，parser 必建
    // 恒非空）：alias_ref 换存活引用只读 location。
    let iter_func_location = alias_ref(for_in_stat.values[0]).base.location;

    let scope = self.find_innermost_scope(iter_func_location);
    // subtyping 已句柄化，判空/解引用契约收进 `subtyping_mut` 访问器（C++ 同契约）。
    let mut r = self
      .subtyping_mut()
      .is_subtype_type_id_type_id_not_null_scope(iter_func, prospective_func, scope.get());

    if !self.is_error_suppressing_location_type_id(iter_func_location, iter_func) {
      for e in &mut r.errors {
        e.location = iter_func_location;
      }
    }

    self.report_errors(take(&mut r.errors));

    if r.normalization_too_complex {
      self.report_error_type_error_data_location(
        NormalizationTooComplex::default().into(),
        &iter_func_location,
      );
    }

    if r.is_subtype {
      return;
    }

    self.explain_error_type_id_type_id_location_subtyping_result(
      iter_func,
      prospective_func,
      iter_func_location,
      &r,
    );
  }
}

impl TypeChecker2 {
  /// cpp `TypeChecker2.cpp:2001`。`expr` 为 arena 存活节点的共享引用
  /// （原 `*mut AstExpr` 形参已引用化，非空由引用类型承载），只读 location、
  /// lookup_type 并向下传给 `test_potential_literal_is_subtype`。
  pub fn test_literal_or_ast_type_is_subtype(
    &mut self,
    expr: &AstExpr,
    expected_type: TypeId,
  ) -> bool {
    let scope = self.find_innermost_scope(expr.base.location);
    let expr_ty = self.lookup_type(expr);

    // self.subtyping 已句柄化（构造期接线指向自有 _subtyping 字段的句柄，
    // C++ 引用成员直译），借用经 `subtyping_mut` 止于本语句，
    // 此刻由本 fn 对 self 的 &mut 独占，单线程下无并存别名；scope 经
    // `get()` 物化为非空共享引用传入子类型链。
    let r = self
      .subtyping_mut()
      .is_subtype_type_id_type_id_not_null_scope(expr_ty, expected_type, scope.get());

    if r.is_subtype {
      return true;
    }

    self.test_potential_literal_is_subtype(expr, expected_type)
  }
}

// `TypeChecker2::tryStripUnionFromNil`（TypeChecker2.cpp:2087-2106）的核心
// 遍历已单源化于 [`crate::functions::try_strip_union_from_nil`]（cpp
// TypeUtils.cpp 同名 static 函数），此处仅做 arena 接线。

impl TypeChecker2 {
  pub fn try_strip_union_from_nil(&self, ty: TypeId) -> Option<TypeId> {
    // SAFETY: self.module 裸指针与类型检查会话同寿（C++ 同契约）；&self 下
    // 经裸指针 place 取 &mut internal_types，与原 C++ const 方法写 arena 等价。
    unsafe { try_strip_union_from_nil(&mut (*self.module).internal_types, ty) }
  }
}

impl TypeChecker2 {
  /// 对应 C++ `TypeChecker2` 的 visitExprName 助手段（`cpp/Analysis/src/TypeChecker2.cpp:2267`）：
  /// 先查 expr 推断类型、剥 nil 再核对索引属性。
  ///
  /// 降 safe 说明：`expr` 原为 `*mut AstExpr`，函数体只把它当存活节点做只读遍历
  /// （`visit_expr`/`lookup_type` 均收引用），故形参直接收窄为 `&AstExpr`，
  /// 裸指针派生留在调用点既有 unsafe 处。
  pub fn visit_expr_name(
    &mut self,
    expr: &AstExpr,
    location: Location,
    prop_name: &str,
    context: ValueContext,
    ast_index_expr_ty: TypeId,
  ) {
    self.visit_expr(expr, ValueContext::RValue);
    let inferred = self.lookup_type(expr);
    let left_type = self.strip_from_nil_and_report(inferred, &location);
    self.check_index_type_from_type(left_type, prop_name, context, location, ast_index_expr_ty);
  }
}
