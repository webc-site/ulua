use alloc::{
  string::{String, ToString},
  vec::Vec,
};
use core::ptr::{NonNull, null_mut};
use std::option::Option;

use ulua_ast::{
  records::{
    ast_array::AstArray,
    ast_expr::AstExpr,
    ast_expr_binary::AstExprBinary,
    ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_nil::AstExprConstantNil,
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate,
    ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal,
    ast_expr_table::{AstExprTable, Item},
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_unary::AstExprUnary,
    ast_expr_varargs::AstExprVarargs,
    ast_node::{self, AstNode},
    ast_stat::AstStat,
    ast_stat_assign::AstStatAssign,
    ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak,
    ast_stat_class::AstStatClass,
    ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_continue::AstStatContinue,
    ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal,
    ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr,
    ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction,
    ast_stat_if::AstStatIf,
    ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction,
    ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn,
    ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction,
    ast_stat_while::AstStatWhile,
    ast_type::AstType,
    ast_type_error::AstTypeError,
    ast_type_function::AstTypeFunction,
    ast_type_group::AstTypeGroup,
    ast_type_intersection::AstTypeIntersection,
    ast_type_list::AstTypeList,
    ast_type_optional::AstTypeOptional,
    ast_type_or_pack::AstTypeOrPack,
    ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_pack_variadic::AstTypePackVariadic,
    ast_type_reference::AstTypeReference,
    ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString,
    ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof,
    ast_type_union::AstTypeUnion,
  },
  rtti::{AstNodeClass, AstNodePtr, ast_node_as_unchecked, ast_node_try_as, ast_node_try_as_ptr},
};
use ulua_common::{fflag, fint, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::value_context::ValueContext,
  functions::{
    begin_type_pack::begin,
    end_type_pack::end_type_pack_id,
    finite::finite,
    first::first,
    follow_type, follow_type_pack,
    get_function_name_as_string::get_function_name_as_string,
    get_type, get_type_pack,
    is_optional::is_optional,
    magic_names::{LUAU_FORCE_CONSTRAINT_SOLVING_INCOMPLETE, LUAU_PRINT},
    size_type_pack::size,
  },
  records::{
    any_type::AnyType as AnyTypeRecord,
    checked_function_call_error::CheckedFunctionCallError,
    checked_function_incorrect_args::CheckedFunctionIncorrectArgs,
    constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
    function_type::FunctionType,
    generic_error::GenericError,
    incorrect_generic_parameter_count::IncorrectGenericParameterCount,
    non_strict_context::NonStrictContext,
    non_strict_function_definition_error::NonStrictFunctionDefinitionError,
    non_strict_type_checker::NonStrictTypeChecker,
    normalization_too_complex::NormalizationTooComplex,
    recursion_counter::RecursionCounter,
    scope::Scope,
    swapped_generic_type_parameter::SwappedGenericTypeParameter,
    symbol::Symbol,
    type_fun::TypeFun,
    type_pack_iterator::TypePackIterator as TypePackIteratorAlias,
    unknown_symbol::{Context, UnknownSymbol},
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    def_id_def::DefId as DefIdAlias, name_type::Name, type_error_data::TypeErrorData,
    type_id::TypeId as TypeIdAlias,
  },
};

impl NonStrictTypeChecker {
  pub fn visit_ast_stat(&mut self, stat: *mut AstStat) -> NonStrictContext {
    // SAFETY: 对照 cpp visit(AstStat*)，调用方契约保证 stat 非空且指向 arena
    // 存活节点；仅此处一次裸解引用换得 &AstNode，后续判别全部走安全引用。
    let node = unsafe { &*stat.cast::<AstNode>() };
    let _pusher = self.push_stack(stat.cast::<AstNode>());
    match node.class_index {
      AstStatBlock::CLASS_INDEX => self.visit_ast_stat_block(stat.cast::<AstStatBlock>()),
      AstStatIf::CLASS_INDEX => self.visit_ast_stat_if(stat.cast::<AstStatIf>()),
      AstStatWhile::CLASS_INDEX => self.visit_ast_stat_while(stat.cast::<AstStatWhile>()),
      AstStatRepeat::CLASS_INDEX => self.visit_ast_stat_repeat(stat.cast::<AstStatRepeat>()),
      AstStatBreak::CLASS_INDEX => {
        self.visit_ast_stat_break(stat.cast::<AstStatBreak>());
        NonStrictContext::new()
      }
      AstStatContinue::CLASS_INDEX => {
        self.visit_ast_stat_continue(stat.cast::<AstStatContinue>());
        NonStrictContext::new()
      }
      AstStatReturn::CLASS_INDEX => self.visit_ast_stat_return(stat.cast::<AstStatReturn>()),
      AstStatExpr::CLASS_INDEX => self.visit_ast_stat_expr(stat.cast::<AstStatExpr>()),
      AstStatLocal::CLASS_INDEX => self.visit_ast_stat_local(stat.cast::<AstStatLocal>()),
      AstStatFor::CLASS_INDEX => self.visit_ast_stat_for(stat.cast::<AstStatFor>()),
      AstStatForIn::CLASS_INDEX => self.visit_ast_stat_for_in(stat.cast::<AstStatForIn>()),
      AstStatAssign::CLASS_INDEX => self.visit_ast_stat_assign(stat.cast::<AstStatAssign>()),
      AstStatCompoundAssign::CLASS_INDEX => {
        self.visit_ast_stat_compound_assign(stat.cast::<AstStatCompoundAssign>())
      }
      AstStatFunction::CLASS_INDEX => self.visit_ast_stat_function(stat.cast::<AstStatFunction>()),
      AstStatLocalFunction::CLASS_INDEX => {
        self.visit_ast_stat_local_function(stat.cast::<AstStatLocalFunction>())
      }
      AstStatTypeAlias::CLASS_INDEX => {
        self.visit_ast_stat_type_alias(stat.cast::<AstStatTypeAlias>())
      }
      AstStatTypeFunction::CLASS_INDEX => {
        self.visit_ast_stat_type_function(stat.cast::<AstStatTypeFunction>());
        NonStrictContext::new()
      }
      AstStatDeclareFunction::CLASS_INDEX => {
        self.visit_ast_stat_declare_function(stat.cast::<AstStatDeclareFunction>())
      }
      AstStatDeclareGlobal::CLASS_INDEX => {
        self.visit_ast_stat_declare_global(stat.cast::<AstStatDeclareGlobal>())
      }
      AstStatDeclareExternType::CLASS_INDEX => {
        self.visit_ast_stat_declare_extern_type(stat.cast::<AstStatDeclareExternType>())
      }
      AstStatClass::CLASS_INDEX => {
        // SAFETY: node.class_index 已经确认为 AstStatClass::CLASS_INDEX
        self.visit_ast_stat_class(unsafe { ast_node_as_unchecked::<AstStatClass>(node) })
      }
      AstStatError::CLASS_INDEX => self.visit_ast_stat_error(stat.cast::<AstStatError>()),
      _ => {
        LUAU_ASSERT!(
          false,
          "NonStrictTypeChecker encountered an unknown statement type"
        );
        // SAFETY: ice 字段在构造期由 C++ NotNull<InternalErrorReporter> 注入，
        // 指向外层检查器上下文持有的报告器，生命周期覆盖整次分析，解引用无别名写。
        self
          .ice
          .get()
          .ice_string("NonStrictTypeChecker encountered an unknown statement type");
        NonStrictContext::new()
      }
    }
  }

  pub fn visit_ast_expr_constant_bool(
    &mut self,
    _expr: *mut AstExprConstantBool,
  ) -> NonStrictContext {
    NonStrictContext::new()
  }

  pub fn visit_ast_expr_constant_number(
    &mut self,
    _expr: *mut AstExprConstantNumber,
  ) -> NonStrictContext {
    NonStrictContext::new()
  }

  pub fn visit_ast_expr_constant_integer(
    &mut self,
    _expr: *mut AstExprConstantInteger,
  ) -> NonStrictContext {
    NonStrictContext::new()
  }

  pub fn visit_ast_expr_constant_string(
    &mut self,
    _expr: *mut AstExprConstantString,
  ) -> NonStrictContext {
    NonStrictContext::new()
  }

  pub fn visit_ast_expr_local_value_context(
    &mut self,
    _local: *mut AstExprLocal,
    _context: ValueContext,
  ) -> NonStrictContext {
    // C++ `visit(AstExprLocal*, ValueContext) { return {}; }`
    NonStrictContext::new()
  }

  /// cpp `NonStrictTypeChecker::visit(AstExprGlobal*, ValueContext)`
  /// (NonStrictTypeChecker.cpp:641)。`global` 为存活引用（非空由类型系统证明）；
  /// 未查得符号时按节点 Location 报 UnknownSymbol。
  pub fn visit_ast_expr_global_value_context(
    &mut self,
    global: &AstExprGlobal,
    context: ValueContext,
  ) -> NonStrictContext {
    // We don't file unknown symbols for LValues.
    if context == ValueContext::LValue {
      return NonStrictContext::new();
    }

    let Some(scope) = self.stack.last().copied() else {
      return NonStrictContext::new();
    };

    let sym = Symbol::from_global(global.name);
    // 栈已句柄化（#24 字段面）：scope 为栈顶 `Handle<Scope>`，由 push_stack 压入，
    // 指向 module.ast_scopes 中 Arc 保活的 Scope，本函数期间无人弹栈或改写作用域树；
    // `get()` 解引用契约集中在 `arena_handle`，只读查表免 unsafe。
    if scope.get().lookup_symbol(sym).is_none() {
      let name_str = global.name.as_str_or_empty();
      let error_data =
        TypeErrorData::UnknownSymbol(UnknownSymbol::new(name_str.to_string(), Context::Binding));
      self.report_error(error_data, &global.base.base.location);
    }

    NonStrictContext::new()
  }

  pub fn visit_ast_expr_varargs(&mut self, _varargs: *mut AstExprVarargs) -> NonStrictContext {
    NonStrictContext::new()
  }

  pub fn visit_ast_expr_call(&mut self, call: &AstExprCall) -> NonStrictContext {
    // visit(call->func, ValueContext::RValue);
    let func_ptr = call.func;
    self.visit_ast_expr_value_context(func_ptr, ValueContext::RValue);

    // for (auto arg : call->args) visit(arg, ValueContext::RValue);
    for &arg in call.args.as_slice() {
      self.visit_ast_expr_value_context(arg, ValueContext::RValue);
    }

    let fresh = NonStrictContext::new();
    // C++ `TypeId* originalCallTy = module->astOriginalCallTypes.find(call->func);`
    // (keyed by `const AstNode*`); `if (!originalCallTy) return fresh;`
    let call_func_node = call.func as *const ast_node::AstNode;
    // SAFETY: module 在 checker 存活期内有效。
    let original_call_ty =
      match unsafe { (*self.module).ast_original_call_types.find(&call_func_node) } {
        Some(v) => v,
        None => return fresh,
      };

    let fn_ty = *original_call_ty;
    // if (auto fn = get<FunctionType>(follow(fnTy)); fn && fn->isCheckedFunction)
    let followed_fn_ty = follow_type::follow(fn_ty);
    let Some(fn_ptr) = get_type::get::<FunctionType>(followed_fn_ty) else {
      return fresh;
    };
    if !fn_ptr.is_checked_function {
      return fresh;
    }

    // Build argument list
    let mut arguments: Vec<*mut AstExpr> =
      Vec::with_capacity(call.args.size + usize::from(call.self_));

    if call.self_ {
      // C++ `if (auto indexExpr = call->func->as<AstExprIndexName>())`
      // SAFETY: call.func 由 AST 父子关系保证存活。
      match ast_node_try_as::<AstExprIndexName>(unsafe {
        &*(call.func as *const ast_node::AstNode)
      }) {
        // expr 已句柄化恒非空；arguments 行走链为既有裸指针 API，经 as_ptr 桥接。
        Some(index_expr) => arguments.push(index_expr.expr.as_ptr()),
        None => {
          // SAFETY: ice 指向 InternalErrorReporter，存活期覆盖 checker。
          self
            .ice
            .get()
            .ice_string("method call expression has no 'self'");
        }
      }
    }

    arguments.extend_from_slice(call.args.as_slice());

    // Collect expected arg types
    let mut arg_types: Vec<TypeIdAlias> = Vec::with_capacity(arguments.len());

    let mut curr: TypePackIteratorAlias = begin(fn_ptr.arg_types);
    let fin: TypePackIteratorAlias = end_type_pack_id(fn_ptr.arg_types);
    while curr != fin {
      let ty: TypeIdAlias = *curr.current();
      arg_types.push(ty);
      curr.advance();
    }

    if let Some(arg_tail) = curr.tail() {
      let followed = follow_type_pack::follow(arg_tail);
      if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(followed) {
        while arg_types.len() < arguments.len() {
          arg_types.push(vtp.ty);
        }
      }
    }

    // SAFETY: call.func 由 AST 父子关系保证存活。
    let function_name = get_function_name_as_string(unsafe { &*call.func }).unwrap_or_default();

    if arguments.len() > arg_types.len() {
      self.report_error(
        TypeErrorData::CheckedFunctionIncorrectArgs(CheckedFunctionIncorrectArgs::new(
          function_name,
          arg_types.len(),
          arguments.len(),
        )),
        &call.base.base.location,
      );
      return fresh;
    }

    let mut fresh_ctx = NonStrictContext::new();
    // arguments 与 arg_types 一一对应，zip 替代索引遍历
    for (&arg, &expected_arg_type) in arguments.iter().zip(arg_types.iter()) {
      // C++ `std::shared_ptr<const NormalizedType> norm = normalizer.normalize(...)`;
      // 归一化失败（过于复杂）时与 C++ 一致报 NormalizationTooComplex 后按非 any 处理
      let norm = self.normalizer.try_normalize(expected_arg_type);
      if norm.is_none() {
        // SAFETY: arg 指向 AST arena 节点。
        let loc = unsafe { (*arg).base.location };
        self.report_error(
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
          &loc,
        );
      }

      let any_ptr = norm
        .as_ref()
        .and_then(|n| get_type::get::<AnyTypeRecord>(n.tops));
      let run_time_error_ty: TypeIdAlias = if any_ptr.is_some() {
        // SAFETY: builtin_types 在 checker 存活期内有效。
        self.builtin_types_ref().never_type
      } else {
        self.get_or_create_negation(expected_arg_type)
      };

      // SAFETY: dfg 在 checker 存活期内有效；arg 是存活 AST 节点。
      let def: DefIdAlias = unsafe { (*self.dfg).get_def(arg) };
      fresh_ctx.add_context(&def, run_time_error_ty);
    }

    let scope = self.find_innermost_scope(call.base.base.location);
    for (i, &arg) in arguments.iter().enumerate() {
      if let Some(run_time_failure_type) =
        // SAFETY: arg 源自上方 arguments 向量（call.args 的 arena 元素或
        // index_expr.expr），仍存活；scope 为 find_innermost_scope 沿模块
        // Scope 树收窄所得、C++ 侧以 NotNull 包装，二者满足被调 unsafe fn
        // 的节点/作用域有效性契约，且此处只读。
        unsafe { self.will_run_time_error(arg, &fresh_ctx, scope) }
      {
        self.report_error(
          TypeErrorData::CheckedFunctionCallError(CheckedFunctionCallError::new(
            arg_types[i],
            run_time_failure_type,
            function_name.clone(),
            i,
          )),
          // SAFETY: arg 是存活 AST 节点。
          unsafe { &(*arg).base.location },
        );
      }
    }

    if arguments.len() < arg_types.len() {
      let mut remaining_args_optional = true;
      for &arg_type in &arg_types[arguments.len()..] {
        remaining_args_optional = remaining_args_optional && is_optional(arg_type);
      }

      if !remaining_args_optional {
        self.report_error(
          TypeErrorData::CheckedFunctionIncorrectArgs(CheckedFunctionIncorrectArgs::new(
            function_name,
            arg_types.len(),
            arguments.len(),
          )),
          &call.base.base.location,
        );
        return fresh_ctx;
      }
    }

    fresh_ctx
  }

  /// cpp `NonStrictTypeChecker::visit(AstExprIndexName*, ValueContext)`
  /// (NonStrictTypeChecker.cpp:779)：仅向下访问被索引对象 `expr`，
  /// `index_name` 为存活引用。
  pub fn visit_ast_expr_index_name_value_context(
    &mut self,
    index_name: &AstExprIndexName,
    context: ValueContext,
  ) -> NonStrictContext {
    // expr 已句柄化恒非空；visit_ast_expr_value_context 为既有裸指针 API，经 as_ptr 桥接。
    self.visit_ast_expr_value_context(index_name.expr.as_ptr(), context)
  }

  /// cpp `NonStrictTypeChecker::visit(AstExprIndexExpr*, ValueContext)`
  /// (NonStrictTypeChecker.cpp:784)：`expr` 与 `index` 子指针按各自上下文
  /// 向下访问后取合取；`index_expr` 为存活引用。
  pub fn visit_ast_expr_index_expr_value_context(
    &mut self,
    index_expr: &AstExprIndexExpr,
    context: ValueContext,
  ) -> NonStrictContext {
    // expr/index 已句柄化恒非空；visit_ast_expr_value_context 为既有裸指针 API，经 as_ptr 桥接。
    let expr = index_expr.expr.as_ptr();
    let index = index_expr.index.as_ptr();

    let expr_context = self.visit_ast_expr_value_context(expr, context);
    let index_context = self.visit_ast_expr_value_context(index, ValueContext::RValue);

    NonStrictContext::disjunction(
      self.builtin_types,
      self.arena,
      &expr_context,
      &index_context,
    )
  }

  pub(crate) fn visit_ast_expr_function(
    &mut self,
    expr_fn: *mut AstExprFunction,
  ) -> NonStrictContext {
    // TODO: should a function being used as an expression generate a context without the arguments?
    let pusher = self.push_stack(expr_fn.as_ast_node());
    // SAFETY: expr_fn 来自 visit_ast_stat_function 的 func 字段或表达式分派的
    // 匹配节点，arena 存活；body 由 parser 恒填充非空（C++ 直接传体同样假定）。
    let mut remainder = unsafe { self.visit_ast_stat_block((*expr_fn).body.as_ptr()) };

    let scope: *mut Scope = match &pusher {
      // 守卫已句柄化：仅还原裸句柄值传参（不解引用），与原 `*mut Scope` 同址。
      Some(p) => p.scope.as_ptr(),
      // SAFETY: module 字段指向本次分析持有的 Module；get_module_scope 返回
      // 模块自身保活的 Arc<Scope> 克隆，临时句柄析构不影响堆上 Scope 存活，
      // 与 find_innermost_scope 的取址模式同一（C++ NotNull{...get()} 行 797）。
      None => unsafe { (*self.module).get_module_scope().as_ref() as *const Scope as *mut Scope },
    };

    // SAFETY: expr_fn 及其 args/debugname/generics/return_annotation 字段均属
    // 存活 arena 节点；scope 要么来自 push_stack（module.ast_scopes 的 Arc 元素）
    // 要么为上方模块 Scope，均无写别名；dfg 字段指向检查前构建完成的
    // DataFlowGraph，local 为其键控的存活 AstLocal。被调
    // will_run_time_error_function_definition 的裸指针契约由此满足。
    unsafe {
      let args = &(*expr_fn).args;
      for local_node in args.iter_nodes() {
        let local = local_node.as_ptr();
        let local_ref = local_node.get();
        if let Some(ty) = self.will_run_time_error_function_definition(local, scope, &remainder) {
          let debugname: String = (*expr_fn).debugname.as_str_or_empty().to_string();
          let arg_name: String = local_ref.name.as_str_or_empty().to_string();
          let loc = local_ref.location;
          self.report_error(
            TypeErrorData::NonStrictFunctionDefinitionError(NonStrictFunctionDefinitionError::new(
              debugname, arg_name, ty,
            )),
            &loc,
          );
        }

        let def = (*self.dfg).get_def_for_local(local);
        remainder.remove(&def);

        self.visit_ast_type(local_ref.annotation);
      }

      self.visit_generics_nodes(&(*expr_fn).generics, &(*expr_fn).generic_packs);

      self.visit_ast_type_pack((*expr_fn).return_annotation.as_ptr());

      if !(*expr_fn).vararg_annotation.is_null() {
        self.visit_ast_type_pack((*expr_fn).vararg_annotation.as_ptr());
      }
    }

    remainder
  }

  pub(crate) fn visit_ast_expr_table(&mut self, table: *mut AstExprTable) -> NonStrictContext {
    // SAFETY: table 由表达式分派在 class_index 命中 AstExprTable 后传入，指向
    // arena 存活节点，块内只读取 items 数组；计数裸指针取自本函数对
    // self.non_strict_recursion_count 的独占借用，_rc 守卫在本函数返回时析构，
    // 期间仅增减该 i32（对照 C++ RecursionCounter RAII 模式）。
    unsafe {
      let mut _rc = Option::None;
      if fflag::LuauAddRecursionCounterToNonStrictTypeChecker.get() {
        _rc = Option::Some(RecursionCounter::recursion_counter_i32(
          &mut self.non_strict_recursion_count,
        ));
        if fint::LuauNonStrictTypeCheckerRecursionLimit.get() > 0
          && self.non_strict_recursion_count >= fint::LuauNonStrictTypeCheckerRecursionLimit.get()
        {
          return NonStrictContext::new();
        }
      }

      let items: AstArray<Item> = (*table).items.clone();
      for item in items.as_slice() {
        if !item.key.is_null() {
          self.visit_ast_expr_value_context(item.key, ValueContext::RValue);
        }
        self.visit_ast_expr_value_context(item.value, ValueContext::RValue);
      }

      NonStrictContext::new()
    }
  }

  pub(crate) fn visit_ast_expr_unary(&mut self, unary: *mut AstExprUnary) -> NonStrictContext {
    // SAFETY: unary 是分派处 class_index 匹配出的存活 AstExprUnary 节点；
    // expr 已句柄化恒非空，此处仅拷贝其子句柄指针后向下递归（既有裸指针 API 桥接）。
    unsafe {
      let expr = (*unary).expr.as_ptr();
      self.visit_ast_expr_value_context(expr, ValueContext::RValue)
    }
  }

  pub(crate) fn visit_ast_expr_binary(&mut self, binary: *mut AstExprBinary) -> NonStrictContext {
    // SAFETY: binary 由表达式分派按 class_index 命中后传入，节点存活；n 借用
    // 仅在块内，用于读出 lhs/rhs 两个 arena 子指针，检查阶段 AST 无并发写。
    unsafe {
      let n = &*binary;
      // left/right 已句柄化恒非空；visit_ast_expr_value_context 为既有裸指针 API，经 as_ptr 桥接。
      let lhs = self.visit_ast_expr_value_context(n.left.as_ptr(), ValueContext::RValue);
      let rhs = self.visit_ast_expr_value_context(n.right.as_ptr(), ValueContext::RValue);
      NonStrictContext::disjunction(self.builtin_types, self.arena, &lhs, &rhs)
    }
  }

  pub(crate) fn visit_ast_expr_type_assertion(
    &mut self,
    type_assertion: *mut AstExprTypeAssertion,
  ) -> NonStrictContext {
    // SAFETY: type_assertion 是分派处 class_index 命中 AstExprTypeAssertion 后
    // 传入的存活 arena 节点；annotation/expr 已句柄化恒非空，槽位读句柄后
    // visit_ast_type/visit_ast_expr_value_context 经 as_ptr 桥接既有裸指针 API。
    let annotation = unsafe { (*type_assertion).annotation };
    self.visit_ast_type(annotation.as_ptr());

    let expr = unsafe { (*type_assertion).expr };
    self.visit_ast_expr_value_context(expr.as_ptr(), ValueContext::RValue)
  }

  pub(crate) fn visit_ast_expr_if_else(&mut self, if_else: *mut AstExprIfElse) -> NonStrictContext {
    // SAFETY: if_else 由表达式分派在 class_index 命中 AstExprIfElse 后传入，
    // 指向存活节点；三子句柄属同一 arena（已句柄化恒非空），
    // visit_ast_expr_value_context 经 as_ptr 桥接既有裸指针 API。三处读取之间无写别名。
    let condition = unsafe { (*if_else).condition };
    let _cond_b = self.visit_ast_expr_value_context(condition.as_ptr(), ValueContext::RValue);
    let true_expr = unsafe { (*if_else).true_expr };
    let then_b = self.visit_ast_expr_value_context(true_expr.as_ptr(), ValueContext::RValue);
    let false_expr = unsafe { (*if_else).false_expr };
    let else_b = self.visit_ast_expr_value_context(false_expr.as_ptr(), ValueContext::RValue);

    NonStrictContext::conjunction(self.builtin_types, self.arena, &then_b, &else_b)
  }

  pub(crate) fn visit_ast_expr_interp_string(
    &mut self,
    interp_string: *mut AstExprInterpString,
  ) -> NonStrictContext {
    // SAFETY: interp_string 由表达式分派 class_index 命中后传入，节点存活；
    // expressions 数组存储在 arena 中，元素逐个作为只读指针传入下层访问器。
    let expressions = unsafe { (*interp_string).expressions };
    for &expr in expressions.as_slice() {
      self.visit_ast_expr_value_context(expr, ValueContext::RValue);
    }

    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_expr_error(&mut self, error: *mut AstExprError) -> NonStrictContext {
    // SAFETY: error 指向 parser 生成的存活 AstExprError arena 节点（分派处按
    // class_index 选定）；error_ref 借用只用于迭代 expressions 数组。
    unsafe {
      let error_ref = &*error;
      for &expr in error_ref.expressions.as_slice() {
        self.visit_ast_expr_value_context(expr, ValueContext::RValue);
      }
    }
    NonStrictContext::new()
  }

  /// cpp `NonStrictTypeChecker::visit(AstExprInstantiate*, ...)`
  /// (NonStrictTypeChecker.cpp:882)：先访问类型实参列表再向下访问被实例化
  /// 表达式；`instantiate` 为存活引用，`type_arguments` 变体载荷恒为 arena
  /// 存活节点，Error 形态对应 cpp 向 `visitAstTypePack` 传 null（早退），跳过。
  pub fn visit_ast_expr_instantiate(
    &mut self,
    instantiate: &AstExprInstantiate,
  ) -> NonStrictContext {
    for param in instantiate.type_arguments.as_slice() {
      // 变体分发替代判空哨兵，Error 形态无可访问节点。
      match *param {
        AstTypeOrPack::Type(t) => self.visit_ast_type(NonNull::from(t).as_ptr()),
        AstTypeOrPack::Pack(p) => self.visit_ast_type_pack(NonNull::from(p).as_ptr()),
        AstTypeOrPack::Error => {}
      }
    }

    self.visit_ast_expr_value_context(instantiate.expr, ValueContext::RValue)
  }

  pub fn visit_ast_type(&mut self, ty: *mut AstType) {
    // SAFETY: 块首判空短路后，ty 指向 arena 存活 AstType 节点（调用方均为
    // parser 填充的父子字段或本分派链）；&AstNode 借用只读 class_index，
    // 各分支下转与 C++ static_cast 布局等价（repr(C) 首字段基址重合）。
    unsafe {
      if ty.is_null() {
        return;
      }

      let node = &*ty.cast::<AstNode>();
      match node.class_index {
        AstTypeReference::CLASS_INDEX => {
          self.visit_ast_type_reference(ty.cast::<AstTypeReference>());
        }
        AstTypeTable::CLASS_INDEX => {
          self.visit_ast_type_table(ty.cast::<AstTypeTable>());
        }
        AstTypeFunction::CLASS_INDEX => {
          self.visit_ast_type_function(ty.cast::<AstTypeFunction>());
        }
        AstTypeTypeof::CLASS_INDEX => {
          self.visit_ast_type_typeof(ty.cast::<AstTypeTypeof>());
        }
        AstTypeUnion::CLASS_INDEX => {
          self.visit_ast_type_union(ty.cast::<AstTypeUnion>());
        }
        AstTypeIntersection::CLASS_INDEX => {
          self.visit_ast_type_intersection(ty.cast::<AstTypeIntersection>());
        }
        AstTypeGroup::CLASS_INDEX => {
          let group = ast_node_as_unchecked::<AstTypeGroup>(node);
          self.visit_ast_type(group.type_);
        }
        AstTypeOptional::CLASS_INDEX
        | AstTypeSingletonBool::CLASS_INDEX
        | AstTypeSingletonString::CLASS_INDEX => {
          // 三类叶节点都不产生约束：`?` 部件在 AST 上没有内层类型成员（cpp 的
          // AstTypeOptional 亦无该成员），单例字面量类型无需再递归。
        }
        AstTypeError::CLASS_INDEX => {
          let error = ast_node_as_unchecked::<AstTypeError>(node);
          for &ty in error.types.as_slice() {
            self.visit_ast_type(ty);
          }
        }
        _ => {
          LUAU_ASSERT!(false);
        }
      }
    }
  }

  pub(crate) fn visit_ast_type_reference(&mut self, ty: *mut AstTypeReference) {
    // SAFETY: ty 由 visit_ast_type 在 ast_node_is::<AstTypeReference> 命中后
    // 传入，指向存活 arena 节点；ty_ref 借用覆盖全函数，检查阶段 AST 只读无别名写。
    let ty_ref = unsafe { &*ty };

    // C++ compares `ty->name` against `kLuauPrint` ("_luau_print") and
    // `kLuauForceConstraintSolvingIncomplete`
    // ("_luau_force_constraint_solving_incomplete") from TypeUtils.h.
    if fflag::DebugLuauMagicTypes.get() {
      let magic_name = ty_ref.name.as_str_or_empty();
      // No further validation is necessary in this case.
      if magic_name == LUAU_PRINT {
        return;
      }

      if magic_name == LUAU_FORCE_CONSTRAINT_SOLVING_INCOMPLETE {
        let error = ConstraintSolvingIncompleteError::default();
        self.report_error(error.into(), &ty_ref.base.base.location);
        return;
      }
    }

    let params = ty_ref.parameters;
    for &param in params.as_slice() {
      // 同上：变体分发替代判空哨兵，Error 形态无可访问节点。
      match param {
        AstTypeOrPack::Type(t) => self.visit_ast_type(NonNull::from(t).as_ptr()),
        AstTypeOrPack::Pack(p) => self.visit_ast_type_pack(NonNull::from(p).as_ptr()),
        AstTypeOrPack::Error => {}
      }
    }

    let scope_ptr = self.find_innermost_scope(ty_ref.base.base.location);
    LUAU_ASSERT!(!scope_ptr.is_null());
    // SAFETY: find_innermost_scope 恒返回模块 Scope 树中的节点（Arc 保活，
    // 起步即取模块 Scope），与 C++ 侧 NotNull/断言（NonStrictTypeChecker.cpp:951）
    // 同一前提；scope 借用只读查询别名表。
    let scope = unsafe { &*scope_ptr };

    let name_str: Name = ty_ref.name.as_str_or_empty().to_string();

    let alias: Option<TypeFun> = if let Some(prefix) = ty_ref.prefix {
      let prefix_str: Name = prefix.as_str_or_empty().to_string();
      scope.lookup_imported_type(&prefix_str, &name_str)
    } else {
      scope.lookup_type(&name_str)
    };

    if let Some(ref alias_ref) = alias {
      let types_required = alias_ref.type_params().len();
      let packs_required = alias_ref.type_pack_params().len();

      let has_default_types = alias_ref
        .type_params()
        .iter()
        .any(|el| el.default_value.is_some());

      let has_default_packs = alias_ref
        .type_pack_params()
        .iter()
        .any(|el| el.default_value.is_some());

      if !ty_ref.has_parameter_list
        && ((!alias_ref.type_params().is_empty() && !has_default_types)
          || (!alias_ref.type_pack_params().is_empty() && !has_default_packs))
      {
        let error = GenericError::new(String::from("Type parameter list is required"));
        self.report_error(error.into(), &ty_ref.base.base.location);
      }

      let mut types_provided = 0usize;
      let mut extra_types = 0usize;
      let mut packs_provided = 0usize;

      for &param in params.as_slice() {
        // cpp `if (param.type) … else if (param.typePack) …`：Error 形态两臂都不进。
        match param {
          AstTypeOrPack::Type(_) => {
            if packs_provided != 0 {
              let error = GenericError::new(String::from(
                "Type parameters must come before type pack parameters",
              ));
              self.report_error(error.into(), &ty_ref.base.base.location);
              continue;
            }

            if types_provided < types_required {
              types_provided += 1;
            } else {
              extra_types += 1;
            }
          }
          AstTypeOrPack::Pack(pack) => {
            let tp = self.lookup_pack_annotation(NonNull::from(pack).as_ptr());
            if !tp.is_some() {
              continue;
            }

            // Safety: 上方 is_none 分支已 continue。
            let tp_id = tp.expect("上方 is_none 分支已 continue，此处必为 Some");
            // C++ `size(*tp) == 1 && finite(*tp) && first(*tp)` — `size`/`finite`
            // default their `TxnLog*` to nullptr; `first` defaults
            // `ignoreHiddenVariadics` to true.
            if types_provided < types_required
              && size(tp_id, None) == 1
              // SAFETY: 同上存活 pack；finite 以 null log 走无事务查询分支。
              && unsafe { finite(tp_id, null_mut()) }
              && first(tp_id, true).is_some()
            {
              types_provided += 1;
            } else {
              packs_provided += 1;
            }
          }
          AstTypeOrPack::Error => {}
        }
      }

      if extra_types != 0 && packs_provided == 0 {
        // Extra types are only collected into a pack if a pack is expected
        if packs_required != 0 {
          packs_provided += 1;
        } else {
          types_provided += extra_types;
        }
      }

      let mut idx = types_provided;
      while idx < types_required {
        if let Some(param) = alias_ref.type_params().get(idx)
          && param.default_value.is_some()
        {
          types_provided += 1;
        }
        idx += 1;
      }

      let mut idx = packs_provided;
      while idx < packs_required {
        if let Some(param) = alias_ref.type_pack_params().get(idx)
          && param.default_value.is_some()
        {
          packs_provided += 1;
        }
        idx += 1;
      }

      if extra_types == 0 && packs_provided + 1 == packs_required {
        packs_provided += 1;
      }

      if types_provided != types_required || packs_provided != packs_required {
        let error = IncorrectGenericParameterCount {
          name: name_str.clone(),
          type_fun: alias_ref.clone(),
          actual_parameters: types_provided,
          actual_pack_parameters: packs_provided,
        };
        self.report_error(error.into(), &ty_ref.base.base.location);
      }
    } else {
      if scope.lookup_pack(&name_str).is_some() {
        let error = SwappedGenericTypeParameter {
          name: String::from(ty_ref.name.as_str_or_empty()),
          kind: SwappedGenericTypeParameter::TYPE,
        };
        self.report_error(error.into(), &ty_ref.base.base.location);
      } else {
        let mut symbol = String::new();
        if let Some(prefix) = ty_ref.prefix {
          let prefix_str = prefix.as_str_or_empty().to_string();
          symbol.push_str(&prefix_str);
          symbol.push('.');
        }
        let name_lossy = ty_ref.name.as_str_or_empty();
        symbol.push_str(name_lossy);

        let error = UnknownSymbol::new(symbol, Context::Type);
        self.report_error(error.into(), &ty_ref.base.base.location);
      }
    }
  }

  pub(crate) fn visit_ast_type_table(&mut self, table: *mut AstTypeTable) {
    // SAFETY: table 由 visit_ast_type 匹配 AstTypeTable 后传入，存活 arena 节点；
    // indexer 判空后解引用，其 index/result 子类型指针由 parser 填充。
    unsafe {
      if !(*table).indexer.is_null() {
        let indexer = (*table).indexer;
        self.visit_ast_type((*indexer).index_type);
        self.visit_ast_type((*indexer).result_type);
      }

      let props = &(*table).props;
      for prop in props.as_slice() {
        self.visit_ast_type(prop.r#type);
      }
    }
  }

  pub(crate) fn visit_ast_type_function(&mut self, function: *mut AstTypeFunction) {
    // SAFETY: function 为 visit_ast_type 匹配 AstTypeFunction 后的存活节点；
    // &mut 借用 arg_types 仅因被调方签名为 &mut AstTypeList，其实现只读遍历，
    // 借用随块结束，无其他并发别名。
    unsafe {
      self.visit_ast_type_list(&mut (*function).arg_types);
      self.visit_ast_type_pack((*function).return_types);
    }
  }

  pub(crate) fn visit_ast_type_typeof(&mut self, type_of: *mut AstTypeTypeof) {
    // SAFETY: type_of 由 visit_ast_type 匹配 AstTypeTypeof 后传入且存活；
    // expr 子指针指向 arena 表达式节点。
    unsafe {
      self.visit_ast_expr_value_context((*type_of).expr, ValueContext::RValue);
    }
  }

  pub(crate) fn visit_ast_type_union(&mut self, union_type: *mut AstTypeUnion) {
    // SAFETY: union_type 为匹配 AstTypeUnion 的存活 arena 节点，
    // types 数组元素由 parser 填充、同属 arena。
    unsafe {
      let types = (*union_type).types;
      for &t in types.as_slice() {
        self.visit_ast_type(t);
      }
    }
  }

  pub(crate) fn visit_ast_type_intersection(
    &mut self,
    intersection_type: *mut AstTypeIntersection,
  ) {
    // SAFETY: intersection_type 由 visit_ast_type 匹配后传入，节点存活；
    // 仅读出 types 数组并逐项向下递归。
    unsafe {
      let types = (*intersection_type).types;
      for &ty in types.as_slice() {
        self.visit_ast_type(ty);
      }
    }
  }

  pub(crate) fn visit_ast_stat_block(&mut self, block: *mut AstStatBlock) -> NonStrictContext {
    LUAU_ASSERT!(!block.is_null());

    let mut _rc: Option<RecursionCounter> = None;
    if fflag::LuauAddRecursionCounterToNonStrictTypeChecker.get() {
      // 计数守卫直接借用 self.non_strict_recursion_count 的 &mut 独占借用，
      // _rc 先于本函数返回析构，构造/丢弃只增减该 i32 本身（构造器已 safe 化）。
      _rc = Some(RecursionCounter::recursion_counter_i32(
        &mut self.non_strict_recursion_count,
      ));
      if fint::LuauNonStrictTypeCheckerRecursionLimit.get() > 0
        && self.non_strict_recursion_count >= fint::LuauNonStrictTypeCheckerRecursionLimit.get()
      {
        return NonStrictContext::new();
      }
    }

    let _stack_pusher = self.push_stack(block.as_ast_node());

    let mut ctx = NonStrictContext::new();

    // SAFETY: block 在块首断言非空且由 visit_ast_stat/函数体分派保证 arena 存活；
    // body/vars 数组与其中 AstStatLocal、AstLocal 均为 parser 分配的节点，
    // local 经 ast_node_try_as_ptr 门面判型+判空一步折叠（cpp `stat->as<AstStatLocal>()`），
    // dfg 字段指向检查前构建完成的 DataFlowGraph 只读表。
    unsafe {
      let block = &*block;
      for stat_node in block.body.iter_nodes().rev() {
        let stat = stat_node.as_ptr();

        if let Some(local) = ast_node_try_as_ptr::<AstStatLocal>(stat) {
          self.visit_ast_stat(stat);
          for &var in &local.vars {
            let def = (*self.dfg).get_def_ast_local(var);
            ctx.remove(&def);
            // C++ `visit(local->annotation)` — `local` here is the loop var (AstLocal).
            self.visit_ast_type((*var).annotation);
          }
        } else {
          let other_ctx = self.visit_ast_stat(stat);
          ctx = NonStrictContext::disjunction(self.builtin_types, self.arena, &other_ctx, &ctx);
        }
      }
    }

    ctx
  }

  pub fn visit_ast_type_list(&mut self, list: &mut AstTypeList) {
    for &t in list.types.as_slice() {
      self.visit_ast_type(t);
    }

    if !list.tail_type.is_null() {
      self.visit_ast_type_pack(list.tail_type);
    }
  }

  pub fn visit_ast_type_pack(&mut self, pack: *mut AstTypePack) {
    if pack.is_null() {
      return;
    }

    // SAFETY: 上方判空后 pack 指向 arena 存活节点；一次裸解引用换得 &AstNode。
    let node = unsafe { &*pack.cast::<AstNode>() };
    match node.class_index {
      AstTypePackExplicit::CLASS_INDEX => {
        self.visit_ast_type_pack_explicit(pack.cast::<AstTypePackExplicit>());
      }
      AstTypePackVariadic::CLASS_INDEX => {
        self.visit_ast_type_pack_variadic(pack.cast::<AstTypePackVariadic>());
      }
      _ => {}
    }
  }

  pub(crate) fn visit_ast_type_pack_explicit(&mut self, tp: *mut AstTypePackExplicit) {
    // SAFETY: tp 由 visit_ast_type_pack 匹配 AstTypePackExplicit 后传入，节点存活；
    // type_list 内嵌于该节点，其 types/tail_type 数组元素均指向 arena 类型节点。
    unsafe {
      let type_list = (*tp).type_list;
      let types = type_list.types;
      for &ty in types.as_slice() {
        self.visit_ast_type(ty);
      }

      let tail_type = type_list.tail_type;
      if !tail_type.is_null() {
        self.visit_ast_type_pack(tail_type);
      }
    }
  }

  pub(crate) fn visit_ast_type_pack_variadic(&mut self, tp: *mut AstTypePackVariadic) {
    // SAFETY: tp 为匹配 AstTypePackVariadic 的存活 arena 节点；
    // variadic_type 子指针由 parser 填充，传入只读的 visit_ast_type。
    unsafe {
      self.visit_ast_type((*tp).variadic_type);
    }
  }

  pub(crate) fn visit_ast_stat_if(&mut self, if_statement: *mut AstStatIf) -> NonStrictContext {
    let if_stmt = unsafe { &*if_statement };
    let cond_b =
      self.visit_ast_expr_value_context(if_stmt.condition.as_ptr(), ValueContext::RValue);
    let then_body = self.visit_ast_stat_block(if_stmt.thenbody.as_ptr());
    let branch_context = if let Some(else_body_node) = if_stmt.elsebody.to_option() {
      let else_body = self.visit_ast_stat(else_body_node.as_ptr());
      NonStrictContext::conjunction(self.builtin_types, self.arena, &then_body, &else_body)
    } else {
      then_body
    };

    NonStrictContext::disjunction(self.builtin_types, self.arena, &cond_b, &branch_context)
  }

  pub(crate) fn visit_ast_stat_while(
    &mut self,
    while_statement: *mut AstStatWhile,
  ) -> NonStrictContext {
    let while_stmt = unsafe { &*while_statement };
    let condition_context =
      self.visit_ast_expr_value_context(while_stmt.condition.as_ptr(), ValueContext::RValue);
    self.visit_ast_stat_block(while_stmt.body.as_ptr());
    let body_context = NonStrictContext::new();
    NonStrictContext::disjunction(
      self.builtin_types,
      self.arena,
      &condition_context,
      &body_context,
    )
  }

  pub(crate) fn visit_ast_stat_repeat(
    &mut self,
    repeat_statement: *mut AstStatRepeat,
  ) -> NonStrictContext {
    let repeat_stmt = unsafe { &*repeat_statement };
    // body/condition 已句柄化为 Node（parser 填充同 arena 节点，非空由类型层
    // 承载），as_ptr 桥交指针形态的 visit 门面（同上方 while 分支）。
    let body_context = self.visit_ast_stat_block(repeat_stmt.body.as_ptr());
    let condition_context =
      self.visit_ast_expr_value_context(repeat_stmt.condition.as_ptr(), ValueContext::RValue);
    NonStrictContext::disjunction(
      self.builtin_types,
      self.arena,
      &body_context,
      &condition_context,
    )
  }

  pub fn visit_ast_stat_break(&mut self, _break_statement: *mut AstStatBreak) {}

  pub fn visit_ast_stat_continue(&mut self, _continue_statement: *mut AstStatContinue) {
    // C++ `visit(AstStatContinue*)` 返回空 `NonStrictContext{}`；
    // 此处无上下文需要构造。
  }

  pub(crate) fn visit_ast_stat_return(
    &mut self,
    return_statement: *mut AstStatReturn,
  ) -> NonStrictContext {
    // SAFETY: return_statement 由 visit_ast_stat 匹配 AstStatReturn 后传入且存活；
    // 引用仅借出 list 数组（元素为 arena 表达式）供本函数遍历。
    let return_statement_ref = unsafe { &*return_statement };
    let list = return_statement_ref.list;
    for &expr in list.as_slice() {
      let _ = self.visit_ast_expr_value_context(expr, ValueContext::RValue);
    }
    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_expr(&mut self, expr: *mut AstStatExpr) -> NonStrictContext {
    // SAFETY: expr 由 visit_ast_stat 在 ast_node_is::<AstStatExpr> 命中后传入，
    // 节点存活；其 expr 子字段已句柄化为非空 Node，as_ptr 仅透传同一 arena 地址
    // 给仍以裸指针身份消费的 visit 门面。
    let inner = unsafe { (*expr).expr };
    self.visit_ast_expr_value_context(inner.as_ptr(), ValueContext::RValue)
  }

  pub(crate) fn visit_ast_stat_local(&mut self, local: *mut AstStatLocal) -> NonStrictContext {
    // SAFETY: local 为 visit_ast_stat 分派匹配的存活 AstStatLocal；
    // local_ref 借用只读遍历 values 数组（元素是 arena 表达式）。
    let local_ref = unsafe { &*local };
    let values = local_ref.values;
    for &rhs in values.as_slice() {
      self.visit_ast_expr_value_context(rhs, ValueContext::RValue);
    }
    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_for(&mut self, for_statement: *mut AstStatFor) -> NonStrictContext {
    // Safety: for_statement 是 visit_ast_stat 分派传入的存活 AstStatFor；
    // var 已句柄化为 Node（parser 恒填充，非空由类型层承载），as_ref().expect
    // 门面随类型折叠；annotation 仍为可空裸指针槽。
    let var_annotation = unsafe { (*for_statement).var.get() }.annotation;
    self.visit_ast_type(var_annotation);

    // from/to 已句柄化为 Node（parser 必建上下界，同上）：cpp 的判空守卫
    // 随类型消失，as_ptr 桥交指针形态门面。
    let from = unsafe { (*for_statement).from.as_ptr() };
    self.visit_ast_expr_value_context(from, ValueContext::RValue);

    let to = unsafe { (*for_statement).to.as_ptr() };
    self.visit_ast_expr_value_context(to, ValueContext::RValue);

    // step 落可空 OptNode（C++ 同样判空），判空后用。
    let step = unsafe { (*for_statement).step.as_ptr() };
    if !step.is_null() {
      self.visit_ast_expr_value_context(step, ValueContext::RValue);
    }

    // body 已句柄化为 Node（parser 非空由类型层承载）。
    let body = unsafe { (*for_statement).body.as_ptr() };
    self.visit_ast_stat_block(body)
  }

  pub(crate) fn visit_ast_stat_for_in(
    &mut self,
    for_in_statement: *mut AstStatForIn,
  ) -> NonStrictContext {
    // SAFETY: for_in_statement 为 visit_ast_stat 分派匹配的存活 AstStatForIn，
    // vars/values/body 字段与数组元素均由 parser 写入 arena。
    let for_in_ref = unsafe { &*for_in_statement };

    // Visit variable annotations
    let vars = &for_in_ref.vars;
    for &var in vars.as_slice() {
      // SAFETY: var 取自上方存活 for_in_ref.vars 数组，指向 parser 分配的
      // AstLocal；annotation 可为空并已判空。
      let annotation = unsafe { (*var).annotation };
      if !annotation.is_null() {
        self.visit_ast_type(annotation);
      }
    }

    // Visit value expressions
    let values = &for_in_ref.values;
    for &rhs in values.as_slice() {
      self.visit_ast_expr_value_context(rhs, ValueContext::RValue);
    }

    // Visit body
    // body 已句柄化为 Node（非空由类型层承载），as_ptr+cast 桥交指针形态门面。
    self.visit_ast_stat(for_in_ref.body.as_ptr().cast::<AstStat>())
  }

  pub(crate) fn visit_ast_stat_assign(&mut self, assign: *mut AstStatAssign) -> NonStrictContext {
    // SAFETY: assign 由 visit_ast_stat 分派保证存活；vars/values 数组元素
    // 均为 parser 分配的 arena 表达式节点，块内只读出指针。
    unsafe {
      let assign_ref = &*assign;
      let vars = assign_ref.vars;
      let values = assign_ref.values;

      for &lhs in vars.as_slice() {
        self.visit_ast_expr_value_context(lhs, ValueContext::LValue);
      }

      for &rhs in values.as_slice() {
        self.visit_ast_expr_value_context(rhs, ValueContext::RValue);
      }
    }

    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_compound_assign(
    &mut self,
    compound_assign: *mut AstStatCompoundAssign,
  ) -> NonStrictContext {
    // SAFETY: compound_assign 指向分派匹配的存活 AstStatCompoundAssign；
    // var/value 已句柄化，as_ptr 仅透传同一 arena 地址给以指针身份消费的 visit 门面。
    unsafe {
      let compound_assign = &*compound_assign;
      let var = compound_assign.var;
      let value = compound_assign.value;

      let _ = self.visit_ast_expr_value_context(var.as_ptr(), ValueContext::LValue);
      let _ = self.visit_ast_expr_value_context(value.as_ptr(), ValueContext::RValue);
    }

    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_function(
    &mut self,
    stat_fn: *mut AstStatFunction,
  ) -> NonStrictContext {
    // SAFETY: stat_fn 为 visit_ast_stat 匹配 AstStatFunction 后的存活 arena 节点；
    // func 已句柄化为 Node（parser 填充同 arena 闭包节点，非空由类型层承载），
    // as_ptr 桥交指针形态的 visit_ast_expr_function。
    let func = unsafe { (*stat_fn).func.as_ptr() };
    self.visit_ast_expr_function(func)
  }

  pub(crate) fn visit_ast_stat_local_function(
    &mut self,
    local_fn: *mut AstStatLocalFunction,
  ) -> NonStrictContext {
    // SAFETY: local_fn 指向分派匹配的存活 AstStatLocalFunction；func 已句柄化为
    // Node，`cast`/`as_ptr` 只改静态类型、不改地址，仍以 *mut AstExpr 桥交
    // 指针形态分派门面。
    unsafe {
      // C++ `visit(localFn->func, ValueContext::RValue)` dispatches via the
      // generic `visit(AstExpr*, ValueContext)` overload (AstExprFunction* upcasts).
      let func = (*local_fn).func.cast::<AstExpr>().as_ptr();
      self.visit_ast_expr_value_context(func, ValueContext::RValue)
    }
  }

  pub(crate) fn visit_ast_stat_type_alias(
    &mut self,
    type_alias: *mut AstStatTypeAlias,
  ) -> NonStrictContext {
    // SAFETY: type_alias 是 visit_ast_stat 分派传入的存活 AstStatTypeAlias；
    // generics/generic_packs/type_ptr 字段由 parser 填充、随父节点存活。
    unsafe {
      let type_alias = &*type_alias;

      self.visit_generics(type_alias.generics, type_alias.generic_packs);
      self.visit_ast_type(type_alias.type_ptr);

      NonStrictContext::new()
    }
  }

  pub fn visit_ast_stat_type_function(&mut self, _type_func: *mut AstStatTypeFunction) {
    // NonStrictContext visit(AstStatTypeFunction* typeFunc) { return {}; }
    // This overload is a no-op in the non-strict type checker.
  }

  pub(crate) fn visit_ast_stat_declare_function(
    &mut self,
    decl_fn: *mut AstStatDeclareFunction,
  ) -> NonStrictContext {
    // SAFETY: decl_fn 由 visit_ast_stat 匹配 AstStatDeclareFunction 后传入，
    // 声明语句及其 generics/params/ret_types 均在 arena 中；params.clone()
    // 拷贝数组句柄后不再触碰原节点。
    unsafe {
      let generics = (*decl_fn).generics;
      let generic_packs = (*decl_fn).generic_packs;
      let params = (*decl_fn).params;
      let ret_types = (*decl_fn).ret_types;

      self.visit_generics(generics, generic_packs);
      self.visit_ast_type_list(&mut params.clone());
      self.visit_ast_type_pack(ret_types);
    }

    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_declare_global(
    &mut self,
    decl_global: *mut AstStatDeclareGlobal,
  ) -> NonStrictContext {
    // SAFETY: decl_global 为分派匹配的存活 AstStatDeclareGlobal；
    // type_ 子指针指向 parser 填充的 arena 类型注解。
    let type_ = unsafe { (*decl_global).type_ };
    self.visit_ast_type(type_);
    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_declare_extern_type(
    &mut self,
    decl_class: *mut AstStatDeclareExternType,
  ) -> NonStrictContext {
    // SAFETY: decl_class 指向 visit_ast_stat 分派匹配的存活
    // AstStatDeclareExternType；indexer 判空后解引用，props 数组存于 arena。
    unsafe {
      let decl_class_ref = &*decl_class;

      if !decl_class_ref.indexer.is_null() {
        let indexer = &*decl_class_ref.indexer;
        self.visit_ast_type(indexer.index_type);
        self.visit_ast_type(indexer.result_type);
      }

      for prop in decl_class_ref.props.as_slice() {
        self.visit_ast_type(prop.ty);
      }

      NonStrictContext::new()
    }
  }

  /// cpp `NonStrictTypeChecker::visit(AstStatClass*, ...)`
  /// (NonStrictTypeChecker.cpp:521)：逐成员访问——属性读类型注解、方法向下
  /// 访问函数表达式；`decl_class` 为存活引用，`members` 变体数组由 parser 填充。
  pub fn visit_ast_stat_class(&mut self, decl_class: &AstStatClass) -> NonStrictContext {
    let members = &decl_class.members;
    for prop in members.as_slice() {
      if let Some(property) = prop.get_if_0() {
        self.visit_ast_type(property.ty);
      } else if let Some(method) = prop.get_if_1() {
        self.visit_ast_expr_function(method.function);
      } else {
        LUAU_ASSERT!(false);
      }
    }

    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_error(&mut self, error: *mut AstStatError) -> NonStrictContext {
    // SAFETY: error 由 visit_ast_stat 匹配 AstStatError 后传入且存活；
    // statements/expressions 数组均由 parser 写入 arena，迭代只读。
    unsafe {
      let error = &*error;
      for &stat in error.statements.as_slice() {
        self.visit_ast_stat(stat);
      }
      for &expr in error.expressions.as_slice() {
        self.visit_ast_expr_value_context(expr, ValueContext::RValue);
      }
    }
    NonStrictContext::new()
  }

  pub fn visit_ast_expr_value_context(
    &mut self,
    expr: *mut AstExpr,
    context: ValueContext,
  ) -> NonStrictContext {
    let mut _rc: Option<RecursionCounter> = None;
    if fflag::LuauAddRecursionCounterToNonStrictTypeChecker.get() {
      // 计数守卫来自对 self 字段的 &mut 独占借用，_rc 于本函数返回时析构，
      // 构造/丢弃只读写该 i32，不触碰 AST 或其余 self 状态（构造器已 safe 化）。
      _rc = Some(RecursionCounter::recursion_counter_i32(
        &mut self.non_strict_recursion_count,
      ));
      if fint::LuauNonStrictTypeCheckerRecursionLimit.get() > 0
        && self.non_strict_recursion_count >= fint::LuauNonStrictTypeCheckerRecursionLimit.get()
      {
        return NonStrictContext::new();
      }
    }

    let _pusher = self.push_stack(expr.as_ast_node());

    // SAFETY: expr 由各调用方（call.args、AST 父子字段、本分派递归、
    // visit_ast_stat_*）传入，恒为 arena 存活且非空的节点——cpp 侧同一指针直接
    // `expr->as<T>()`（NonStrictTypeChecker.cpp:546-604）作相同非空前提；
    // 检查阶段 AST 只读无写别名。此处为分派链唯一的裸解引用收口点：物化
    // `&AstNode` 后，class_index 读取与各子类下转全部走安全门面。
    let node = unsafe { &*expr.as_ast_node() };
    match node.class_index {
      AstExprGroup::CLASS_INDEX => {
        let group = unsafe { ast_node_as_unchecked::<AstExprGroup>(node) };
        self.visit_ast_expr_group_value_context(group, context)
      }
      AstExprConstantNil::CLASS_INDEX => {
        self.visit_ast_expr_constant_nil(expr.cast::<AstExprConstantNil>())
      }
      AstExprConstantBool::CLASS_INDEX => {
        self.visit_ast_expr_constant_bool(expr.cast::<AstExprConstantBool>())
      }
      AstExprConstantNumber::CLASS_INDEX => {
        self.visit_ast_expr_constant_number(expr.cast::<AstExprConstantNumber>())
      }
      AstExprConstantInteger::CLASS_INDEX => {
        self.visit_ast_expr_constant_integer(expr.cast::<AstExprConstantInteger>())
      }
      AstExprConstantString::CLASS_INDEX => {
        self.visit_ast_expr_constant_string(expr.cast::<AstExprConstantString>())
      }
      AstExprLocal::CLASS_INDEX => {
        self.visit_ast_expr_local_value_context(expr.cast::<AstExprLocal>(), context)
      }
      AstExprGlobal::CLASS_INDEX => {
        let global = unsafe { ast_node_as_unchecked::<AstExprGlobal>(node) };
        self.visit_ast_expr_global_value_context(global, context)
      }
      AstExprVarargs::CLASS_INDEX => self.visit_ast_expr_varargs(expr.cast::<AstExprVarargs>()),
      AstExprCall::CLASS_INDEX => {
        // SAFETY: class_index 命中 AstExprCall，&mut 引用仅存活于本次实参求值，
        // expr 指向 arena 存活节点；检查阶段 AST 只读，无并发别名写。
        self.visit_ast_expr_call(unsafe { &mut *(expr.cast::<AstExprCall>()) })
      }
      AstExprIndexName::CLASS_INDEX => {
        let index_name = unsafe { ast_node_as_unchecked::<AstExprIndexName>(node) };
        self.visit_ast_expr_index_name_value_context(index_name, context)
      }
      AstExprIndexExpr::CLASS_INDEX => {
        let index_expr = unsafe { ast_node_as_unchecked::<AstExprIndexExpr>(node) };
        self.visit_ast_expr_index_expr_value_context(index_expr, context)
      }
      AstExprFunction::CLASS_INDEX => self.visit_ast_expr_function(expr.cast::<AstExprFunction>()),
      AstExprTable::CLASS_INDEX => self.visit_ast_expr_table(expr.cast::<AstExprTable>()),
      AstExprUnary::CLASS_INDEX => self.visit_ast_expr_unary(expr.cast::<AstExprUnary>()),
      AstExprBinary::CLASS_INDEX => self.visit_ast_expr_binary(expr.cast::<AstExprBinary>()),
      AstExprTypeAssertion::CLASS_INDEX => {
        self.visit_ast_expr_type_assertion(expr.cast::<AstExprTypeAssertion>())
      }
      AstExprIfElse::CLASS_INDEX => self.visit_ast_expr_if_else(expr.cast::<AstExprIfElse>()),
      AstExprInterpString::CLASS_INDEX => {
        self.visit_ast_expr_interp_string(expr.cast::<AstExprInterpString>())
      }
      AstExprError::CLASS_INDEX => self.visit_ast_expr_error(expr.cast::<AstExprError>()),
      AstExprInstantiate::CLASS_INDEX => {
        let instantiate = unsafe { ast_node_as_unchecked::<AstExprInstantiate>(node) };
        self.visit_ast_expr_instantiate(instantiate)
      }
      _ => {
        LUAU_ASSERT!(false);
        // SAFETY: ice 字段为构造期注入的 NotNull 报告器，指向外层上下文持有、
        // 覆盖本次分析全程的 InternalErrorReporter。
        self
          .ice
          .get()
          .ice_string("NonStrictTypeChecker encountered an unknown expression type");
        NonStrictContext::new()
      }
    }
  }

  /// cpp `NonStrictTypeChecker::visit(AstExprGroup*, ValueContext)`
  /// (NonStrictTypeChecker.cpp:606)：括号表达式直接向下访问 `expr` 子节点；
  /// `group` 为存活引用。
  pub fn visit_ast_expr_group_value_context(
    &mut self,
    group: &AstExprGroup,
    context: ValueContext,
  ) -> NonStrictContext {
    // expr 已句柄化；visit_ast_expr_value_context 为既有裸指针 API，经 as_ptr 桥接。
    self.visit_ast_expr_value_context(group.expr.as_ptr(), context)
  }

  pub fn visit_ast_expr_constant_nil(
    &mut self,
    _expr: *mut AstExprConstantNil,
  ) -> NonStrictContext {
    NonStrictContext::new()
  }
}
