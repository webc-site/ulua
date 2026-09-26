//! `TypeChecker2::visitCall`（TypeChecker2.cpp:1608-1930）。
use alloc::{format, string::String, vec::Vec};
use core::ptr::NonNull;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_index_name::AstExprIndexName,
    ast_node::AstNode, location::Location,
  },
  rtti::ast_node_try_as,
};
use ulua_common::{
  fflag,
  records::{dense_hash_set::DenseHashSet, variant::Variant2},
};

use crate::{
  enums::{normalization_result::NormalizationResult, value::Value},
  functions::{
    extend_type_pack::extend_type_pack,
    find_unique_types_ast_utils::find_unique_types_exprs as find_unique_types,
    flatten_type_pack::flatten_type_pack_id, follow_type,
    get_parameter_extents::get_parameter_extents, get_type, is_optional::is_optional,
    is_variadic_type_pack::is_variadic, report_available_overloads::report_available_overloads,
    should_suppress_errors_type_utils::should_suppress_errors,
  },
  records::{
    ambiguous_function_call::AmbiguousFunctionCall,
    any_type::AnyType,
    arena_handle::Handle,
    cannot_call_non_function::CannotCallNonFunction,
    count_mismatch::{CountMismatch, CountMismatchContext},
    function_type::FunctionType,
    generic_error::GenericError,
    internal_error::InternalError,
    intersection_type::IntersectionType,
    magic_function_type_check_context::MagicFunctionTypeCheckContext,
    multiple_nonviable_overloads::MultipleNonviableOverloads,
    never_type::NeverType,
    normalization_too_complex::NormalizationTooComplex,
    optional_value_access::OptionalValueAccess,
    overload_resolution::OverloadResolution,
    overload_resolver::OverloadResolver,
    txn_log::TxnLog,
    type_checker_2::TypeChecker2,
    type_pack::TypePack,
    union_type::UnionType,
  },
  type_aliases::{
    error_type::ErrorType, error_vec::ErrorVec, module_name_type::ModuleName,
    type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId,
  },
};
impl TypeChecker2 {
  /// call 的 func 表达式位置（call.func 为 AST arena 内有效节点指针，会话同寿）。
  fn call_func_location(call: &AstExprCall) -> Location {
    // Safety: `call.func` 是 parser 分配 arena 节点时写入 AstExprCall.func 的被调
    // 子表达式指针（对应 C++ `AstExprCall::func`），格式正确的 call 节点上恒非空；
    // 指向物随 AST arena 与 check 会话同寿。此处仅读一次 `.base.location`，无写。
    unsafe { (*call.func).base.location }
  }

  /// 上报重载不匹配与参数数量不匹配错误（雷同逻辑抽自 visitCall 两处解析路径）。
  /// 返回 true 表示已处理，调用方应提前返回。
  fn report_overload_failures(
    &mut self,
    resolver: &OverloadResolver<'_>,
    errors: &mut ErrorVec,
    module_name: &ModuleName,
    args_pack: TypePackId,
    arg_exprs: &[*mut AstExpr],
    call_site: Location,
    func_loc: Location,
    result: &OverloadResolution,
  ) -> bool {
    if result.incompatible_overloads.len() == 1 {
      for (ty, reasons) in result.incompatible_overloads.iter() {
        match reasons {
          Variant2::V0(reasonings) => {
            for reason in reasonings.iter() {
              resolver.report_errors(
                errors,
                *ty,
                func_loc,
                module_name,
                args_pack,
                arg_exprs,
                reason,
              );
            }
          }
          Variant2::V1(errors) => {
            self.report_errors(errors.clone());
          }
        }
      }
      return true;
    }

    if result.incompatible_overloads.len() > 1 {
      let (arg_head, _) = flatten_type_pack_id(args_pack);
      let mut overloads_to_report = Vec::new();
      for (overload_ty, _) in result.incompatible_overloads.iter() {
        if !self.is_error_suppressing_location_type_id(call_site, *overload_ty) {
          overloads_to_report.push(*overload_ty);
        }
      }
      if !overloads_to_report.is_empty() {
        self.report_error_type_error_data_location(
          TypeErrorData::MultipleNonviableOverloads(MultipleNonviableOverloads::new(
            arg_head.len(),
          )),
          &call_site,
        );
        report_available_overloads(errors, call_site, module_name, &overloads_to_report);
      }
      return true;
    }

    if result.arity_mismatches.len() == 1 {
      let mismatch_ty = follow_type::follow(result.arity_mismatches[0]);
      if let Some(mismatch_fn) = get_type::get::<FunctionType>(mismatch_ty) {
        let is_variadic = is_variadic(mismatch_fn.arg_types);
        // Safety: `get_parameter_extents` 为清单外 unsafe fn，契约要求 `log`/`tp`
        // 有效：`TxnLog::empty()` 返回进程寿只读单例地址（见 txn_log_empty 的
        // Sync 证成），`mismatch_fn.arg_types` 是 `get_type_id` 取出的存活
        // FunctionType 节点所持有的 pack id，本次调用只读遍历该 pack。
        let (min_params, opt_max_params) =
          unsafe { get_parameter_extents(TxnLog::empty(), mismatch_fn.arg_types, true) };
        let (arg_head, _) = flatten_type_pack_id(args_pack);
        self.report_error_type_error_data_location(
          TypeErrorData::CountMismatch(CountMismatch {
            expected: min_params,
            maximum: opt_max_params,
            actual: arg_head.len(),
            context: CountMismatchContext::Arg,
            is_variadic,
            function: String::new(),
          }),
          &func_loc,
        );
        return true;
      }
    }

    if !result.arity_mismatches.is_empty() {
      let (arg_head, _) = flatten_type_pack_id(args_pack);
      self.report_error_type_error_data_location(
        TypeErrorData::GenericError(GenericError::new(format!(
          "No overload for function accepts {} arguments.",
          arg_head.len()
        ))),
        &func_loc,
      );
      report_available_overloads(errors, func_loc, module_name, &result.arity_mismatches);
      return true;
    }

    false
  }

  /// 上报非函数类型调用错误（雷同逻辑抽自 visitCall 两处路径）。
  fn report_non_function_errors(&mut self, fn_ty: TypeId, func_loc: Location) {
    let norm = self.normalizer.try_normalize(fn_ty);
    let hit_limits = norm.as_ref().is_none_or(|norm| {
      self.normalizer.is_inhabited_normalized_type(norm.as_ref()) == NormalizationResult::HitLimits
    });
    if hit_limits {
      self.report_error_type_error_data_location(
        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
        &func_loc,
      );
    }

    if norm
      .as_ref()
      .is_none_or(|norm| !norm.should_suppress_errors())
    {
      self.report_error_type_error_data_location(
        TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: fn_ty }),
        &func_loc,
      );
    }
  }

  pub fn visit_call(&mut self, call: &AstExprCall) {
    // Safety: `self.module` 是构造期以 NotNull 语义注入的会话独占 Module（对应
    // C++ TypeChecker2 的 `Module*` 成员），比 checker 长寿。此处建立覆盖整个
    // 函数体的 `&mut` 借用，经它顺序读写 errors/internal_types/ast_* 各 map。
    // 部分被调方法（如 lookup_type）按 C++ 对象图惯例再经同一裸指针临时建立
    // 借用：Module 是单一可变对象、全程单线程时序串行，任一时刻只有一处在写。
    let module = unsafe { &mut *self.module };
    let func_node = call.func as *const AstNode;
    let Some(original_call_ty) = module.ast_original_call_types.find(&func_node).copied() else {
      return;
    };

    let mut fn_ty = follow_type::follow(original_call_ty);
    if get_type::get::<AnyType>(fn_ty).is_some()
      || get_type::get::<ErrorType>(fn_ty).is_some()
      || get_type::get::<NeverType>(fn_ty).is_some()
    {
      return;
    }

    if is_optional(fn_ty) {
      // Safety: 清单外 unsafe fn `should_suppress_errors` 要求 normalizer 指针有效
      // 且 `ty` 为存活类型 id：`&mut self.normalizer` 由 checker 字段的活借用转出，
      // 调用期间 checker 不重入该字段；`fn_ty` 已 follow 且经上方分支排除了
      // Any/Error/Never，是类型 arena 内存活节点。
      match Value::from(unsafe { should_suppress_errors(&mut self.normalizer, fn_ty) }) {
        Value::Suppress => {}
        Value::NormalizationFailed => {
          self.report_error_type_error_data_location(
            TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
            &Self::call_func_location(call),
          );
          self.report_error_type_error_data_location(
            TypeErrorData::OptionalValueAccess(OptionalValueAccess { optional: fn_ty }),
            &Self::call_func_location(call),
          );
        }
        Value::DoNotSuppress => {
          self.report_error_type_error_data_location(
            TypeErrorData::OptionalValueAccess(OptionalValueAccess { optional: fn_ty }),
            &Self::call_func_location(call),
          );
        }
      }
      return;
    }

    if fflag::LuauExplicitTypeInstantiationSupport.get() && call.type_arguments.size != 0 {
      self.check_type_instantiation(
        &call.base,
        fn_ty,
        &call.base.base.location,
        call.type_arguments,
      );
    }

    // AstNode 基座位于 AstExprCall 偏移 0（`base.base`），该引用地址与 C++ 里
    // 直接以 `call` 指针作 map key 的取值一致，无需裸指针转类型。
    let call_node = &call.base.base as *const AstNode;
    if let Some(selected_overload_ty) = module.ast_overload_resolved_types.find(&call_node).copied()
    {
      let scope = self.find_innermost_scope(call.base.base.location);
      // `self.subtyping` 字段已句柄化（Option<Handle<Subtyping>>，构造期
      // `wire_self_pointers` 指向 checker 内嵌 `_subtyping`）：`subtyping_mut`
      // 收口判空与解引用契约，借用止于本语句，与原裸指针 `&mut` 重建同构；
      // `scope` 为非空模块作用域树指针。
      let mut result = self
        .subtyping_mut()
        .is_subtype_type_id_type_id_not_null_scope(original_call_ty, selected_overload_ty, scope);
      if result.is_subtype {
        fn_ty = follow_type::follow(selected_overload_ty);
      }
      // C++: isErrorSuppressing 时把错误位置重定位到调用处（TypeChecker2.cpp:1818-1823）。
      if result.is_error_suppressing {
        for e in &mut result.errors {
          e.location = call.base.base.location;
        }
      }
      self.report_errors(result.errors);
      if result.normalization_too_complex {
        self.report_error_type_error_data_location(
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
          &Self::call_func_location(call),
        );
        return;
      }
    }

    let Some(fty) = get_type::get::<FunctionType>(fn_ty) else {
      let mut args = TypePack::empty();
      let mut arg_exprs: Vec<*mut AstExpr> = Vec::new();

      // The `call->self` prelude in C++ `visitCall` runs before the
      // FunctionType/else split (TypeChecker2.cpp:1624-1634), so the
      // method receiver `self` must be prepended onto `args` here too.
      if call.self_ {
        // Safety: `call.func` 是 parser 写入 AstExprCall 的非空子表达式指针，指向
        // arena 存活节点（与 `call_func_location` 同一来源）；此处只读一次 AstNode
        // 头部做 RTTI 判别，非 IndexName 的情形由下方 `let Some(..) else` 分支兜底。
        let index_expr = ast_node_try_as::<AstExprIndexName>(unsafe { &(*call.func).base });
        let Some(index_expr) = index_expr else {
          self.report_error_type_error_data_location(
            TypeErrorData::InternalError(InternalError {
              message: "method call expression has no 'self'".into(),
            }),
            &call.base.base.location,
          );
          return;
        };

        // index_expr.expr 已句柄化恒非空：.get() 安全借用仅供 lookup_type 一次
        // 只读递归；arg_exprs 行走链为既有裸指针 API，经 as_ptr 桥接。
        args.head.push(self.lookup_type(index_expr.expr.get()));
        arg_exprs.push(index_expr.expr.as_ptr());
      }

      let args_slice = call.args.as_slice();
      for (idx, &arg_expr) in args_slice.iter().enumerate() {
        arg_exprs.push(arg_expr);
        let is_last = idx + 1 == args_slice.len();

        if is_last
          && let Some(last_arg_pack) = module
            .ast_type_packs
            .find(&(arg_expr as *const AstExpr))
            .copied()
        {
          let (last_arg_head, last_arg_tail) = flatten_type_pack_id(last_arg_pack);
          args.head.extend(last_arg_head);
          args.tail = last_arg_tail;
          continue;
        }

        if let Some(arg_ty) = module.ast_types.find(&(arg_expr as *const AstExpr)) {
          args.head.push(*arg_ty);
        } else if is_last {
          // Safety: `self.builtin_types.as_ptr()` 为构造期以 NotNull 语义注入的会话级指针
          // （对应 C++ `NotNull<BuiltinTypes>` 成员），在 visit_call 全程有效且
          // 无人改写；此处只读取其 `any_type_pack` 这一个 id 字段。
          args.tail = Some(self.builtin_types.get().any_type_pack);
        } else {
          // Safety: 同上来源——`self.builtin_types.as_ptr()` 是构造注入且比 checker 长寿的
          // 非空指针，本语句仅读 `any_type` 字段，无任何写操作。
          args.head.push(self.builtin_types.get().any_type);
        }
      }

      let args_pack = module.internal_types.add_type_pack_t(args);
      let scope = self.find_innermost_scope(call.base.base.location);
      // Safety: `OverloadResolver::new` 为 unsafe fn，契约是各裸指针非空且存续期内
      // 有效：builtin_types/type_function_runtime/limits/ice 均为构造期注入的会话级
      // 指针（`new` 只把 `limits` 转为共享借用、不取得所有权，limits 由会话拥有并
      // 全程存活；ice 源自 unifier_state）；
      // scope 为 find_innermost_scope 返回的模块作用域树内非空指针；
      // `&mut module.internal_types`、`&mut self.normalizer` 由本作用域活借用就地
      // 转成。resolver 存续期间的读写全程单线程串行，无第二处并发持有同一对象。
      let mut resolver = unsafe {
        OverloadResolver::new(
          self.builtin_types,
          Handle::from_mut(&mut module.internal_types),
          &mut self.normalizer,
          self.type_function_runtime.as_ptr(),
          scope,
          self.ice.as_ptr(),
          self.limits.as_ptr(),
          call.base.base.location,
        )
      };
      let mut unique_types: DenseHashSet<TypeId> = DenseHashSet::default();
      // find_unique_types 已降 safe：集合/map 直传引用；arg_exprs 各元素为 arena
      // 存活节点的判型解引用在被调方逐元素窄块内证成。
      find_unique_types(&mut unique_types, &arg_exprs, &module.ast_types);

      let func_loc = Self::call_func_location(call);
      let result = resolver.resolve_overload(
        fn_ty,
        args_pack,
        func_loc,
        &mut unique_types as *mut DenseHashSet<TypeId>,
        false,
      );
      if !result.ok.is_empty() {
        if result.ok.len() > 1 {
          self.report_error_type_error_data_location(
            TypeErrorData::AmbiguousFunctionCall(AmbiguousFunctionCall::new(fn_ty, args_pack)),
            &call.base.base.location,
          );
        }
        return;
      }

      if self.report_overload_failures(
        &resolver,
        &mut module.errors,
        &module.name,
        args_pack,
        &arg_exprs,
        call.base.base.location,
        func_loc,
        &result,
      ) {
        return;
      }

      if !result.non_functions.is_empty() {
        self.report_non_function_errors(fn_ty, func_loc);
      } else if get_type::get::<IntersectionType>(fn_ty).is_none()
        && get_type::get::<UnionType>(fn_ty).is_none()
      {
        self.report_error_type_error_data_location(
          TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: fn_ty }),
          &func_loc,
        );
      }
      return;
    };

    let args_slice = call.args.as_slice();
    let mut arg_exprs: Vec<*mut AstExpr> = Vec::new();
    let self_offset = if call.self_ { 1 } else { 0 };
    let use_bidirectional_args =
      fty.generics.is_empty() && fty.generic_packs.is_empty() && !args_slice.is_empty();
    let params_head = if use_bidirectional_args {
      // Safety: 清单外 unsafe fn `extend_type_pack` 契约要求 arena/builtin_types 与
      // `pack` 有效：arena 来自函数头部 `module` 借用的字段可变引用；builtin_types
      // 为构造注入的 NotNull 会话指针；`fty.arg_types` 是 `get_type_id` 取出的存活
      // FunctionType 节点内嵌的 pack id，函数体内对其只读并追加新 pack 到 arena。
      unsafe {
        extend_type_pack(
          &mut module.internal_types,
          Handle::from_ptr(self.builtin_types.as_ptr()),
          fty.arg_types,
          args_slice.len() + self_offset,
          Vec::new(),
        )
      }
      .head
    } else {
      Vec::new()
    };

    let mut args = TypePack::empty();

    if call.self_ {
      // Safety: `call.func` 与 `call_func_location` 读取的是同一非空 arena 节点
      // 指针；这里只取一次 AstNode 头部供 RTTI 判别，类型不符走下方错误分支。
      let index_expr = ast_node_try_as::<AstExprIndexName>(unsafe { &(*call.func).base });
      let Some(index_expr) = index_expr else {
        self.report_error_type_error_data_location(
          TypeErrorData::InternalError(InternalError {
            message: "method call expression has no 'self'".into(),
          }),
          &call.base.base.location,
        );
        return;
      };

      // index_expr.expr 已句柄化恒非空：.get() 安全借用仅供 lookup_type 一次
      // 只读递归；arg_exprs 行走链为既有裸指针 API，经 as_ptr 桥接。
      args.head.push(self.lookup_type(index_expr.expr.get()));
      arg_exprs.push(index_expr.expr.as_ptr());
    }

    for (idx, &arg_expr) in args_slice.iter().enumerate() {
      let is_last = idx + 1 == args_slice.len();

      if is_last
        && let Some(last_arg_pack) = module
          .ast_type_packs
          .find(&(arg_expr as *const AstExpr))
          .copied()
      {
        let (last_arg_head, last_arg_tail) = flatten_type_pack_id(last_arg_pack);
        args.head.extend(last_arg_head);
        args.tail = last_arg_tail;
        continue;
      }

      // Safety: `arg_expr` 是 parser 写入 `call.args` 数组的实参节点地址，非空且
      // 与 AST arena 同寿；此 &AstExpr 引用交给 lookup_type 只做类型查询。
      let arg_expr_type = self.lookup_type(unsafe { &*arg_expr });
      arg_exprs.push(arg_expr);
      if use_bidirectional_args
        && idx + self_offset < params_head.len()
        && !self.is_error_suppressing_location_type_id(
          // Safety: 同上，`arg_expr` 指向的存活节点，仅再读一次 base.location 字段。
          unsafe { (*arg_expr).base.location },
          arg_expr_type,
        )
      {
        // Safety: 清单外 unsafe fn `test_literal_or_ast_type_is_subtype` 要求
        // `expr` 指向存活 AST 节点且 `expected_type` 为有效类型 id：`arg_expr` 即
        // 上文的 arena 活节点；`params_head[_]` 由刚才 `extend_type_pack` 从该
        // arena 读出/派生，均随会话存活。
        unsafe {
          self.test_literal_or_ast_type_is_subtype(arg_expr, params_head[idx + self_offset])
        };
        args.head.push(params_head[idx + self_offset]);
      } else {
        args.head.push(arg_expr_type);
      }
    }

    // 复用函数头部的 `module` 借用（与 `(*self.module)` 同一对象），无需再次解引用裸指针。
    let args_tp = module.internal_types.add_type_pack_t(args.clone());
    if let Some(original_ftv) = get_type::get::<FunctionType>(follow_type::follow(original_call_ty))
      && let Some(magic) = original_ftv.magic.as_ref()
    {
      let scope = self.find_innermost_scope(call.base.base.location);
      // Safety: 本块的不安全性只来自三处 `NonNull::new_unchecked`，其指针皆非空：
      // `self as *mut _` 派生自 `&mut self`，天然非空；`self.builtin_types.as_ptr()` 为构造
      // 期 NotNull 注入的会话指针；`scope` 从模块根作用域起步（find_innermost_scope
      // 永不返回空）。`magic.type_check` 本身是安全 fn 指针字段；上下文 `&` 借用
      // 仅存活于本语句，magic 回调经 `typechecker` 裸指针重访 self 属 C++ 同构
      // 用法，单线程时序串行、回调返回后无保留别名。
      let used_magic = unsafe {
        (magic.type_check)(&MagicFunctionTypeCheckContext {
          typechecker: NonNull::new_unchecked(self as *mut TypeChecker2),
          builtin_types: NonNull::new_unchecked(self.builtin_types.as_ptr()),
          call_site: call,
          arguments: args_tp,
          check_scope: NonNull::new_unchecked(scope),
        })
      };

      if used_magic {
        return;
      }
    }

    if args.tail.is_none() {
      let actual = args.head.len();
      // Safety: `get_parameter_extents` 的 log/tp 契约在传参处即满足——
      // `TxnLog::empty()` 为进程寿只读单例，`fty.arg_types` 是 fty 所在
      // FunctionType 活节点内嵌 pack id，此处只读遍历（include_hidden 为 false）。
      let (min_params, opt_max_params) =
        unsafe { get_parameter_extents(TxnLog::empty(), fty.arg_types, false) };

      if actual < min_params || opt_max_params.is_some_and(|max_params| actual > max_params) {
        self.report_error_type_error_data_location(
          TypeErrorData::CountMismatch(CountMismatch {
            expected: min_params,
            maximum: opt_max_params,
            actual,
            context: CountMismatchContext::Arg,
            is_variadic: is_variadic(fty.arg_types),
            function: String::new(),
          }),
          &Self::call_func_location(call),
        );
        return;
      }
    }

    let args_pack = module.internal_types.add_type_pack_t(args);
    let scope = self.find_innermost_scope(call.base.base.location);
    // Safety: 与前述非函数分支构造 resolver 时的传参完全同源——会话级
    // builtin_types/type_function_runtime/limits/ice 皆构造期注入非空，scope 落在
    // 模块作用域树内，arena 与 normalizer 由 `module`/`self` 的活借用转出的裸地址；
    // `new` 内 limits 仅转为共享借用（不 clone、不取得所有权），resolver 后续使用
    // 全程单线程串行。
    let mut resolver = unsafe {
      OverloadResolver::new(
        self.builtin_types,
        Handle::from_mut(&mut module.internal_types),
        &mut self.normalizer,
        self.type_function_runtime.as_ptr(),
        scope,
        self.ice.as_ptr(),
        self.limits.as_ptr(),
        call.base.base.location,
      )
    };
    let mut unique_types: DenseHashSet<TypeId> = DenseHashSet::default();
    // 同上：被调已降 safe，直传引用。
    find_unique_types(&mut unique_types, &arg_exprs, &module.ast_types);

    let func_loc = Self::call_func_location(call);
    let result = resolver.resolve_overload(
      fn_ty,
      args_pack,
      func_loc,
      &mut unique_types as *mut DenseHashSet<TypeId>,
      false,
    );

    if !result.potential_overloads.is_empty() {
      self.report_error_type_error_data_location(
        TypeErrorData::InternalError(InternalError {
          message: "Internal error: outstanding free or blocked type in function call".into(),
        }),
        &call.base.base.location,
      );
    }

    if !result.ok.is_empty() {
      if result.ok.len() > 1 {
        self.report_error_type_error_data_location(
          TypeErrorData::AmbiguousFunctionCall(AmbiguousFunctionCall::new(fn_ty, args_pack)),
          &call.base.base.location,
        );
      }

      self.lookup_type(&call.base);
      return;
    }

    if self.report_overload_failures(
      &resolver,
      &mut module.errors,
      &module.name,
      args_pack,
      &arg_exprs,
      call.base.base.location,
      func_loc,
      &result,
    ) {
      return;
    }

    if !result.non_functions.is_empty() {
      self.report_non_function_errors(fn_ty, func_loc);
    }
  }
}
