use alloc::{
  string::{String, ToString},
  vec::Vec,
};
use std::option::Option;

use ulua_ast::{
  enums::{
    ast_expr_ref::AstExprRef, ast_stat_ref::AstStatRef, ast_type_pack_ref::AstTypePackRef,
    ast_type_ref::AstTypeRef,
  },
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
    ast_expr_error::AstExprError, ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal, ast_expr_group::AstExprGroup, ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate, ast_expr_interp_string::AstExprInterpString,
    ast_expr_table::AstExprTable, ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_unary::AstExprUnary, ast_local::AstLocal, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock, ast_stat_class::AstStatClass,
    ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn, ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_while::AstStatWhile, ast_type::AstType, ast_type_function::AstTypeFunction,
    ast_type_intersection::AstTypeIntersection, ast_type_list::AstTypeList,
    ast_type_or_pack::AstTypeOrPack, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_pack_variadic::AstTypePackVariadic,
    ast_type_reference::AstTypeReference, ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion,
  },
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
    arena_handle::{alias_opt, alias_ref},
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

impl<'a> NonStrictTypeChecker<'a> {
  /// cpp `visit(AstStat*)`：`stat` 为 arena 存活节点引用（调用方经
  /// `as_stat_ref` 分派或块遍历取得），判别与子类下转全部走
  /// `AstStatRef` 安全枚举，无裸指针。
  pub fn visit_ast_stat(&mut self, stat: &AstStat) -> NonStrictContext {
    let node: &AstNode = &stat.base;
    let _pusher = self.push_stack(node);
    match stat.as_stat_ref() {
      AstStatRef::Block(block) => self.visit_ast_stat_block(block),
      AstStatRef::If(if_statement) => self.visit_ast_stat_if(if_statement),
      AstStatRef::While(while_statement) => self.visit_ast_stat_while(while_statement),
      AstStatRef::Repeat(repeat_statement) => self.visit_ast_stat_repeat(repeat_statement),
      // C++ `visit(AstStatBreak*)` 为 no-op（原 visit_ast_stat_break 空体桩并入本臂）。
      AstStatRef::Break(_) => NonStrictContext::new(),
      // C++ `visit(AstStatContinue*)` 返回空 `NonStrictContext{}`（原
      // visit_ast_stat_continue 空体桩并入本臂）。
      AstStatRef::Continue(_) => NonStrictContext::new(),
      AstStatRef::Return(return_statement) => self.visit_ast_stat_return(return_statement),
      AstStatRef::Expr(expr) => self.visit_ast_stat_expr(expr),
      AstStatRef::Local(local) => self.visit_ast_stat_local(local),
      AstStatRef::For(for_statement) => self.visit_ast_stat_for(for_statement),
      AstStatRef::ForIn(for_in_statement) => self.visit_ast_stat_for_in(for_in_statement),
      AstStatRef::Assign(assign) => self.visit_ast_stat_assign(assign),
      AstStatRef::CompoundAssign(compound_assign) => {
        self.visit_ast_stat_compound_assign(compound_assign)
      }
      AstStatRef::Function(stat_fn) => self.visit_ast_stat_function(stat_fn),
      AstStatRef::LocalFunction(local_fn) => self.visit_ast_stat_local_function(local_fn),
      AstStatRef::TypeAlias(type_alias) => self.visit_ast_stat_type_alias(type_alias),
      // `visit(AstStatTypeFunction*)` 该重载在 non-strict checker 中为 no-op
      // （cpp NonStrictTypeChecker.cpp，原 visit_ast_stat_type_function 空体桩并入本臂）。
      AstStatRef::TypeFunction(_) => NonStrictContext::new(),
      AstStatRef::DeclareFunction(decl_fn) => self.visit_ast_stat_declare_function(decl_fn),
      AstStatRef::DeclareGlobal(decl_global) => self.visit_ast_stat_declare_global(decl_global),
      AstStatRef::DeclareExternType(decl_class) => {
        self.visit_ast_stat_declare_extern_type(decl_class)
      }
      AstStatRef::DeclareClass(cls) => self.visit_ast_stat_class(cls),
      AstStatRef::Error(error) => self.visit_ast_stat_error(error),
    }
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

  pub fn visit_ast_expr_call(&mut self, call: &AstExprCall) -> NonStrictContext {
    // visit(call->func, ValueContext::RValue);
    // `call.func` 为 ast record 未句柄化字段（恒非空，parser 填充同 arena 节点），
    // 经 `alias_ref` 门面（解引用契约集中在 arena_handle）换得存活引用。
    self.visit_ast_expr_value_context(alias_ref(call.func), ValueContext::RValue);

    // for (auto arg : call->args) visit(arg, ValueContext::RValue);
    for arg in call.args.iter_nodes() {
      self.visit_ast_expr_value_context(arg, ValueContext::RValue);
    }

    let fresh = NonStrictContext::new();
    // C++ `TypeId* originalCallTy = module->astOriginalCallTypes.find(call->func);`
    // (keyed by `const AstNode*`); `if (!originalCallTy) return fresh;`
    // 指针仅作映射键（身份语义），不触碰解引用。
    let call_func_node = call.func as *const AstNode;
    let original_call_ty = match self
      .module_ref()
      .ast_original_call_types
      .find(&call_func_node)
    {
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
    let mut arguments: Vec<&AstExpr> = Vec::with_capacity(call.args.size + usize::from(call.self_));

    if call.self_ {
      // C++ `if (auto indexExpr = call->func->as<AstExprIndexName>())`
      if let AstExprRef::IndexName(index_expr) = alias_ref(call.func).as_expr_ref() {
        arguments.push(index_expr.expr.get());
      } else {
        self
          .ice
          .get()
          .ice_string("method call expression has no 'self'");
      }
    }

    arguments.extend(call.args.iter_nodes());

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
      if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(followed)
        && arg_types.len() < arguments.len()
      {
        // 变参尾部按参数个数补齐同类型（仅扩张，故 resize 不会截断）。
        arg_types.resize(arguments.len(), vtp.ty);
      }
    }

    let function_name = get_function_name_as_string(alias_ref(call.func)).unwrap_or_default();

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
        let loc = arg.base.location;
        self.report_error(
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex),
          &loc,
        );
      }

      let any_ptr = norm
        .as_ref()
        .and_then(|n| get_type::get::<AnyTypeRecord>(n.tops));
      let run_time_error_ty: TypeIdAlias = if any_ptr.is_some() {
        self.builtin_types_ref().never_type
      } else {
        self.get_or_create_negation(expected_arg_type)
      };

      // 指针仅作 DFG 映射键（身份语义）；dfg 解引用经 `dfg_ref` 收口。
      let def: DefIdAlias = self.dfg_ref().get_def(arg as *const AstExpr);
      fresh_ctx.add_context(&def, run_time_error_ty);
    }

    let scope = self.find_innermost_scope(call.base.base.location);
    for (i, &arg) in arguments.iter().enumerate() {
      if let Some(run_time_failure_type) = self.will_run_time_error(arg, &fresh_ctx, scope.get()) {
        self.report_error(
          TypeErrorData::CheckedFunctionCallError(CheckedFunctionCallError::new(
            arg_types[i],
            run_time_failure_type,
            function_name.clone(),
            i,
          )),
          &arg.base.location,
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
    // expr 已句柄化恒非空：`Node::get` 直出存活只读引用。
    self.visit_ast_expr_value_context(index_name.expr.get(), context)
  }

  /// cpp `NonStrictTypeChecker::visit(AstExprIndexExpr*, ValueContext)`
  /// (NonStrictTypeChecker.cpp:784)：`expr` 与 `index` 子指针按各自上下文
  /// 向下访问后取合取；`index_expr` 为存活引用。
  pub fn visit_ast_expr_index_expr_value_context(
    &mut self,
    index_expr: &AstExprIndexExpr,
    context: ValueContext,
  ) -> NonStrictContext {
    // expr/index 已句柄化恒非空：`Node::get` 直出存活只读引用。
    let expr_context = self.visit_ast_expr_value_context(index_expr.expr.get(), context);
    let index_context =
      self.visit_ast_expr_value_context(index_expr.index.get(), ValueContext::RValue);

    NonStrictContext::disjunction(
      self.builtin_types,
      self.arena,
      &expr_context,
      &index_context,
    )
  }

  pub(crate) fn visit_ast_expr_function(&mut self, expr_fn: &AstExprFunction) -> NonStrictContext {
    // TODO: should a function being used as an expression generate a context without the arguments?
    let pusher = self.push_stack(&expr_fn.base.base);
    // body 已句柄化为非空 `Node<AstStatBlock>`：`get` 直出存活引用。
    let mut remainder = self.visit_ast_stat_block(expr_fn.body.get());

    // 守卫已句柄化：栈顶作用域经 `Handle::get` 换存活引用；无栈时取模块根
    // 作用域（`get_module_scope` 返回模块自身保活的 `Arc<Scope>` 克隆，本地
    // 绑定在整个函数内存活，引用不悬垂；与 find_innermost_scope 的取根同一）。
    let module_scope;
    let scope: &Scope = match &pusher {
      Some(p) => p.scope.get(),
      None => {
        module_scope = self.module_ref().get_module_scope();
        &module_scope
      }
    };

    // args 已句柄化为 `Nodes<AstLocal>`：iter_nodes + get 直出存活引用；
    // will_run_time_error_function_definition 亦改为收引用，全链路无 unsafe。
    for local_node in expr_fn.args.iter_nodes() {
      let local_ref = local_node.get();
      if let Some(ty) = self.will_run_time_error_function_definition(local_ref, scope, &remainder) {
        let debugname: String = expr_fn.debugname.as_str_or_empty().to_string();
        let arg_name: String = local_ref.name.as_str_or_empty().to_string();
        let loc = local_ref.location;
        self.report_error(
          TypeErrorData::NonStrictFunctionDefinitionError(NonStrictFunctionDefinitionError::new(
            debugname, arg_name, ty,
          )),
          &loc,
        );
      }

      // 指针仅作 DFG 映射键（身份语义）；dfg 解引用经 `dfg_ref` 收口。
      let def = self
        .dfg_ref()
        .get_def_for_local(local_ref as *const AstLocal);
      remainder.remove(&def);

      self.visit_ast_type(alias_opt(local_ref.annotation));
    }

    self.visit_generics_nodes(&expr_fn.generics, &expr_fn.generic_packs);

    self.visit_ast_type_pack(expr_fn.return_annotation.get());

    // vararg_annotation 为可空 `OptNode`：Option 形态承接 cpp 的判空。
    self.visit_ast_type_pack(expr_fn.vararg_annotation.get());

    remainder
  }

  pub(crate) fn visit_ast_expr_table(&mut self, table: &AstExprTable) -> NonStrictContext {
    // items 是可 Copy 的内嵌字段，一次拷出数组句柄后不再触碰原节点；
    // 计数裸指针取自本函数对 self.non_strict_recursion_count 的独占借用，
    // _rc 守卫在本函数返回时析构，期间仅增减该 i32
    // （对照 C++ RecursionCounter RAII 模式）。
    let items = table.items.clone();

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

    for item in items.as_slice() {
      // key 为可空槽位（cpp `if (item.key)`），alias_opt 折叠为 Option。
      if let Some(key) = alias_opt(item.key) {
        self.visit_ast_expr_value_context(key, ValueContext::RValue);
      }
      self.visit_ast_expr_value_context(alias_ref(item.value), ValueContext::RValue);
    }

    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_expr_unary(&mut self, unary: &AstExprUnary) -> NonStrictContext {
    // expr 已句柄化恒非空：`Node::get` 直出存活只读引用。
    self.visit_ast_expr_value_context(unary.expr.get(), ValueContext::RValue)
  }

  pub(crate) fn visit_ast_expr_binary(&mut self, binary: &AstExprBinary) -> NonStrictContext {
    // left/right 已句柄化恒非空：`Node::get` 直出存活只读引用。
    let lhs = self.visit_ast_expr_value_context(binary.left.get(), ValueContext::RValue);
    let rhs = self.visit_ast_expr_value_context(binary.right.get(), ValueContext::RValue);
    NonStrictContext::disjunction(self.builtin_types, self.arena, &lhs, &rhs)
  }

  pub(crate) fn visit_ast_expr_type_assertion(
    &mut self,
    type_assertion: &AstExprTypeAssertion,
  ) -> NonStrictContext {
    // annotation/expr 已句柄化恒非空：`Node::get` 直出存活只读引用。
    self.visit_ast_type(Some(type_assertion.annotation.get()));
    self.visit_ast_expr_value_context(type_assertion.expr.get(), ValueContext::RValue)
  }

  pub(crate) fn visit_ast_expr_if_else(&mut self, if_else: &AstExprIfElse) -> NonStrictContext {
    // 三子句柄已句柄化恒非空：`Node::get` 直出存活只读引用。
    let _cond_b = self.visit_ast_expr_value_context(if_else.condition.get(), ValueContext::RValue);
    let then_b = self.visit_ast_expr_value_context(if_else.true_expr.get(), ValueContext::RValue);
    let else_b = self.visit_ast_expr_value_context(if_else.false_expr.get(), ValueContext::RValue);

    NonStrictContext::conjunction(self.builtin_types, self.arena, &then_b, &else_b)
  }

  pub(crate) fn visit_ast_expr_interp_string(
    &mut self,
    interp_string: &AstExprInterpString,
  ) -> NonStrictContext {
    // expressions 数组元素为 arena 表达式节点，iter_nodes 安全门面逐个给出引用。
    for expr in interp_string.expressions.iter_nodes() {
      self.visit_ast_expr_value_context(expr, ValueContext::RValue);
    }

    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_expr_error(&mut self, error: &AstExprError) -> NonStrictContext {
    // expressions 数组元素为 arena 表达式节点，iter_nodes 安全门面逐个给出引用。
    for expr in error.expressions.iter_nodes() {
      self.visit_ast_expr_value_context(expr, ValueContext::RValue);
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
        AstTypeOrPack::Type(t) => self.visit_ast_type(Some(t)),
        AstTypeOrPack::Pack(p) => self.visit_ast_type_pack(Some(p)),
        AstTypeOrPack::Error => {}
      }
    }

    // expr 为 ast record 未句柄化字段（恒非空，parser 填充同 arena 节点），
    // 经 `alias_ref` 门面（解引用契约集中在 arena_handle）换得存活引用。
    self.visit_ast_expr_value_context(alias_ref(instantiate.expr), ValueContext::RValue)
  }

  /// cpp `visit(AstType*)`：可空形态由 `Option<&AstType>` 承载（None 即 cpp 的
  /// nullptr 早退）；分派与子类下转走 `AstTypeRef` 安全枚举。
  pub(crate) fn visit_ast_type(&mut self, ty: Option<&AstType>) {
    let Some(ty) = ty else {
      return;
    };

    match ty.as_type_ref() {
      AstTypeRef::Reference(reference) => self.visit_ast_type_reference(reference),
      AstTypeRef::Table(table) => self.visit_ast_type_table(table),
      AstTypeRef::Function(function) => self.visit_ast_type_function(function),
      AstTypeRef::Typeof(type_of) => self.visit_ast_type_typeof(type_of),
      AstTypeRef::Union(union_type) => self.visit_ast_type_union(union_type),
      AstTypeRef::Intersection(intersection_type) => {
        self.visit_ast_type_intersection(intersection_type)
      }
      AstTypeRef::Group(group) => {
        self.visit_ast_type(alias_opt(group.type_));
      }
      AstTypeRef::Optional(_) | AstTypeRef::SingletonBool(_) | AstTypeRef::SingletonString(_) => {
        // 三类叶节点都不产生约束：`?` 部件在 AST 上没有内层类型成员（cpp 的
        // AstTypeOptional 亦无该成员），单例字面量类型无需再递归。
      }
      AstTypeRef::Error(error) => {
        for &ty in error.types.as_slice() {
          self.visit_ast_type(alias_opt(ty));
        }
      }
    }
  }

  pub(crate) fn visit_ast_type_reference(&mut self, ty: &AstTypeReference) {
    // C++ compares `ty->name` against `kLuauPrint` ("_luau_print") and
    // `kLuauForceConstraintSolvingIncomplete`
    // ("_luau_force_constraint_solving_incomplete") from TypeUtils.h.
    if fflag::DebugLuauMagicTypes.get() {
      let magic_name = ty.name.as_str_or_empty();
      // No further validation is necessary in this case.
      if magic_name == LUAU_PRINT {
        return;
      }

      if magic_name == LUAU_FORCE_CONSTRAINT_SOLVING_INCOMPLETE {
        let error = ConstraintSolvingIncompleteError;
        self.report_error(error.into(), &ty.base.base.location);
        return;
      }
    }

    let params = ty.parameters;
    for param in params.iter() {
      // 同上：变体分发替代判空哨兵，Error 形态无可访问节点。
      match param {
        AstTypeOrPack::Type(t) => self.visit_ast_type(Some(t)),
        AstTypeOrPack::Pack(p) => self.visit_ast_type_pack(Some(p)),
        AstTypeOrPack::Error => {}
      }
    }

    // find_innermost_scope 恒返回模块 Scope 树中的节点（起步即取模块根），
    // 句柄解引用契约集中在 arena_handle。
    let scope = self.find_innermost_scope(ty.base.base.location).get();

    let name_str: Name = ty.name.as_str_or_empty().to_string();

    let alias: Option<TypeFun> = if let Some(prefix) = &ty.prefix {
      let prefix_str: Name = prefix.as_str_or_empty().to_string();
      scope.lookup_imported_type(&prefix_str, &name_str)
    } else {
      scope.lookup_type(&name_str)
    };

    if let Some(ref alias_fun) = alias {
      let types_required = alias_fun.type_params().len();
      let packs_required = alias_fun.type_pack_params().len();

      let has_default_types = alias_fun
        .type_params()
        .iter()
        .any(|el| el.default_value.is_some());

      let has_default_packs = alias_fun
        .type_pack_params()
        .iter()
        .any(|el| el.default_value.is_some());

      if !ty.has_parameter_list
        && ((!alias_fun.type_params().is_empty() && !has_default_types)
          || (!alias_fun.type_pack_params().is_empty() && !has_default_packs))
      {
        let error = GenericError::new(String::from("Type parameter list is required"));
        self.report_error(error.into(), &ty.base.base.location);
      }

      let mut types_provided = 0usize;
      let mut extra_types = 0usize;
      let mut packs_provided = 0usize;

      for param in params.iter() {
        // cpp `if (param.type) … else if (param.typePack) …`：Error 形态两臂都不进。
        match param {
          AstTypeOrPack::Type(_) => {
            if packs_provided != 0 {
              let error = GenericError::new(String::from(
                "Type parameters must come before type pack parameters",
              ));
              self.report_error(error.into(), &ty.base.base.location);
              continue;
            }

            if types_provided < types_required {
              types_provided += 1;
            } else {
              extra_types += 1;
            }
          }
          AstTypeOrPack::Pack(pack) => {
            let tp = self.lookup_pack_annotation(pack);
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
              // finite 已 Option<&TxnLog> 化：None 即无事务查询分支，tp_id 存活 pack。
              && finite(tp_id, None)
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
        if let Some(param) = alias_fun.type_params().get(idx)
          && param.default_value.is_some()
        {
          types_provided += 1;
        }
        idx += 1;
      }

      let mut idx = packs_provided;
      while idx < packs_required {
        if let Some(param) = alias_fun.type_pack_params().get(idx)
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
          type_fun: alias_fun.clone(),
          actual_parameters: types_provided,
          actual_pack_parameters: packs_provided,
        };
        self.report_error(error.into(), &ty.base.base.location);
      }
    } else {
      if scope.lookup_pack(&name_str).is_some() {
        let error = SwappedGenericTypeParameter {
          name: String::from(ty.name.as_str_or_empty()),
          kind: SwappedGenericTypeParameter::TYPE,
        };
        self.report_error(error.into(), &ty.base.base.location);
      } else {
        let mut symbol = String::new();
        if let Some(prefix) = &ty.prefix {
          let prefix_str = prefix.as_str_or_empty().to_string();
          symbol.push_str(&prefix_str);
          symbol.push('.');
        }
        let name_lossy = ty.name.as_str_or_empty();
        symbol.push_str(name_lossy);

        let error = UnknownSymbol::new(symbol, Context::Type);
        self.report_error(error.into(), &ty.base.base.location);
      }
    }
  }

  pub(crate) fn visit_ast_type_table(&mut self, table: &AstTypeTable) {
    // indexer 为可空槽位（cpp 判空后解引用），句柄 `get` 折叠为 Option；
    // index/result 为恒非空句柄，`get` 直接给出共享引用。
    if let Some(indexer) = table.indexer.get() {
      self.visit_ast_type(Some(indexer.index_type.get()));
      self.visit_ast_type(Some(indexer.result_type.get()));
    }

    for prop in table.props.as_slice() {
      self.visit_ast_type(alias_opt(prop.r#type));
    }
  }

  pub(crate) fn visit_ast_type_function(&mut self, function: &AstTypeFunction) {
    self.visit_ast_type_list(&function.arg_types);
    self.visit_ast_type_pack(alias_opt(function.return_types));
  }

  pub(crate) fn visit_ast_type_typeof(&mut self, type_of: &AstTypeTypeof) {
    // expr 为 ast record 未句柄化字段（恒非空，parser 填充同 arena 节点），
    // 经 `alias_ref` 门面（解引用契约集中在 arena_handle）换得存活引用。
    self.visit_ast_expr_value_context(alias_ref(type_of.expr), ValueContext::RValue);
  }

  pub(crate) fn visit_ast_type_union(&mut self, union_type: &AstTypeUnion) {
    // types 数组元素由 parser 填充、同属 arena；alias_opt 折叠为可空 Option，
    // 与原「指针传入 visit_ast_type 后判空」的容错语义逐格等价。
    for &t in union_type.types.as_slice() {
      self.visit_ast_type(alias_opt(t));
    }
  }

  pub(crate) fn visit_ast_type_intersection(&mut self, intersection_type: &AstTypeIntersection) {
    // types 数组元素由 parser 填充、同属 arena；alias_opt 折叠为可空 Option，
    // 与原「指针传入 visit_ast_type 后判空」的容错语义逐格等价。
    for &ty in intersection_type.types.as_slice() {
      self.visit_ast_type(alias_opt(ty));
    }
  }

  pub(crate) fn visit_ast_stat_block(&mut self, block: &AstStatBlock) -> NonStrictContext {
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

    let _stack_pusher = self.push_stack(&block.base.base);

    let mut ctx = NonStrictContext::new();

    // body 为句柄化 `Nodes<AstStat>`：iter_nodes + get 直出存活引用；
    // `as_stat_ref` 安全分派替代已退役的指针门面判型下转。
    for stat_node in block.body.iter_nodes().rev() {
      let stat = stat_node.get();

      // cpp `stat->as<AstStatLocal>()`：AstStatLocal 为叶节点，class_index
      // 分派与精确判型等价。
      if let AstStatRef::Local(local) = stat.as_stat_ref() {
        self.visit_ast_stat(stat);
        for var in local.vars.iter_nodes() {
          // 指针仅作 DFG 映射键（身份语义）；dfg 解引用经 `dfg_ref` 收口。
          let def = self.dfg_ref().get_def_ast_local(var as *const AstLocal);
          ctx.remove(&def);
          // C++ `visit(local->annotation)` — `local` here is the loop var (AstLocal).
          // annotation 可为空，alias_opt 折叠后交 visit_ast_type 的 Option 入口。
          self.visit_ast_type(alias_opt(var.annotation));
        }
      } else {
        let other_ctx = self.visit_ast_stat(stat);
        ctx = NonStrictContext::disjunction(self.builtin_types, self.arena, &other_ctx, &ctx);
      }
    }

    ctx
  }

  pub fn visit_ast_type_list(&mut self, list: &AstTypeList) {
    // types 数组元素由 parser 填充、同属 arena；alias_opt 折叠为可空 Option。
    for &t in list.types.as_slice() {
      self.visit_ast_type(alias_opt(t));
    }

    self.visit_ast_type_pack(alias_opt(list.tail_type));
  }

  /// cpp `visit(AstTypePack*)`：可空形态由 `Option<&AstTypePack>` 承载
  /// （None 即 cpp 的 nullptr 早退）；分派走 `AstTypePackRef` 安全枚举。
  pub(crate) fn visit_ast_type_pack(&mut self, pack: Option<&AstTypePack>) {
    let Some(pack) = pack else {
      return;
    };

    match pack.as_pack_ref() {
      AstTypePackRef::Explicit(tp) => self.visit_ast_type_pack_explicit(tp),
      AstTypePackRef::Variadic(variadic) => self.visit_ast_type_pack_variadic(variadic),
      AstTypePackRef::Generic(_) => {}
    }
  }

  pub(crate) fn visit_ast_type_pack_explicit(&mut self, tp: &AstTypePackExplicit) {
    // type_list 是可 Copy 的内嵌字段，一次拷出数组句柄后不再触碰原节点，
    // 其 types/tail_type 数组元素均指向 arena 类型节点。
    let type_list = tp.type_list;
    for &ty in type_list.types.as_slice() {
      self.visit_ast_type(alias_opt(ty));
    }

    self.visit_ast_type_pack(alias_opt(type_list.tail_type));
  }

  pub(crate) fn visit_ast_type_pack_variadic(&mut self, tp: &AstTypePackVariadic) {
    // variadic_type 子指针由 parser 填充，alias_opt 折叠后传入只读的 visit_ast_type。
    self.visit_ast_type(alias_opt(tp.variadic_type));
  }

  pub(crate) fn visit_ast_stat_if(&mut self, if_statement: &AstStatIf) -> NonStrictContext {
    // condition/thenbody 已句柄化恒非空（`Node::get` 直出引用）；elsebody 为
    // 可空 OptNode，`to_option` 折叠后按 cpp 判空语义分流。
    let cond_b =
      self.visit_ast_expr_value_context(if_statement.condition.get(), ValueContext::RValue);
    let then_body = self.visit_ast_stat_block(if_statement.thenbody.get());
    let branch_context = if let Some(else_body_node) = if_statement.elsebody.to_option() {
      let else_body = self.visit_ast_stat(else_body_node.get());
      NonStrictContext::conjunction(self.builtin_types, self.arena, &then_body, &else_body)
    } else {
      then_body
    };

    NonStrictContext::disjunction(self.builtin_types, self.arena, &cond_b, &branch_context)
  }

  pub(crate) fn visit_ast_stat_while(
    &mut self,
    while_statement: &AstStatWhile,
  ) -> NonStrictContext {
    // condition/body 已句柄化恒非空：`Node::get` 直出存活只读引用。
    let condition_context =
      self.visit_ast_expr_value_context(while_statement.condition.get(), ValueContext::RValue);
    self.visit_ast_stat_block(while_statement.body.get());
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
    repeat_statement: &AstStatRepeat,
  ) -> NonStrictContext {
    // body/condition 已句柄化恒非空：`Node::get` 直出存活只读引用。
    let body_context = self.visit_ast_stat_block(repeat_statement.body.get());
    let condition_context =
      self.visit_ast_expr_value_context(repeat_statement.condition.get(), ValueContext::RValue);
    NonStrictContext::disjunction(
      self.builtin_types,
      self.arena,
      &body_context,
      &condition_context,
    )
  }

  pub(crate) fn visit_ast_stat_return(
    &mut self,
    return_statement: &AstStatReturn,
  ) -> NonStrictContext {
    // list 数组元素为 arena 表达式，iter_nodes 安全门面逐个给出引用。
    for expr in return_statement.list.iter_nodes() {
      let _ = self.visit_ast_expr_value_context(expr, ValueContext::RValue);
    }
    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_expr(&mut self, expr: &AstStatExpr) -> NonStrictContext {
    // 内层 expr 已句柄化为非空 Node，`get` 直出存活只读引用。
    self.visit_ast_expr_value_context(expr.expr.get(), ValueContext::RValue)
  }

  pub(crate) fn visit_ast_stat_local(&mut self, local: &AstStatLocal) -> NonStrictContext {
    // values 数组元素为 arena 表达式，iter_nodes 安全门面逐个给出引用。
    for rhs in local.values.iter_nodes() {
      self.visit_ast_expr_value_context(rhs, ValueContext::RValue);
    }
    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_for(&mut self, for_statement: &AstStatFor) -> NonStrictContext {
    // var/from/to/step/body 均已句柄化：var 取 AstLocal 读注解；step 落可空
    // OptNode（C++ 同样判空）；其余 `get` 直出存活引用。
    let var = for_statement.var.get();
    self.visit_ast_type(alias_opt(var.annotation));

    // from/to 已句柄化为 Node（parser 必建上下界）：cpp 的判空守卫随类型消失。
    self.visit_ast_expr_value_context(for_statement.from.get(), ValueContext::RValue);
    self.visit_ast_expr_value_context(for_statement.to.get(), ValueContext::RValue);

    // step 落可空 OptNode（C++ 同样判空），判空后用。
    if let Some(step) = for_statement.step.get() {
      self.visit_ast_expr_value_context(step, ValueContext::RValue);
    }

    self.visit_ast_stat_block(for_statement.body.get())
  }

  pub(crate) fn visit_ast_stat_for_in(
    &mut self,
    for_in_statement: &AstStatForIn,
  ) -> NonStrictContext {
    // vars/values/body 字段与数组元素均由 parser 写入 arena。

    // Visit variable annotations
    for var in for_in_statement.vars.iter_nodes() {
      // annotation 可为空，alias_opt 折叠后交 visit_ast_type 的 Option 入口。
      self.visit_ast_type(alias_opt(var.annotation));
    }

    // Visit value expressions
    for rhs in for_in_statement.values.iter_nodes() {
      self.visit_ast_expr_value_context(rhs, ValueContext::RValue);
    }

    // Visit body
    // body 已句柄化为 Node（非空由类型层承载）；经 `base` 上转 `&AstStat`
    // （repr(C) 基址重合，与原 `as_ptr().cast::<AstStat>()` 同址）。
    let body_stat: &AstStat = &for_in_statement.body.get().base;
    self.visit_ast_stat(body_stat)
  }

  pub(crate) fn visit_ast_stat_assign(&mut self, assign: &AstStatAssign) -> NonStrictContext {
    // vars/values 数组元素为 parser 分配的 arena 表达式节点，
    // iter_nodes 安全门面逐个给出引用。
    for lhs in assign.vars.iter_nodes() {
      self.visit_ast_expr_value_context(lhs, ValueContext::LValue);
    }

    for rhs in assign.values.iter_nodes() {
      self.visit_ast_expr_value_context(rhs, ValueContext::RValue);
    }

    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_compound_assign(
    &mut self,
    compound_assign: &AstStatCompoundAssign,
  ) -> NonStrictContext {
    // var/value 已句柄化恒非空：`Node::get` 直出存活只读引用。
    let _ = self.visit_ast_expr_value_context(compound_assign.var.get(), ValueContext::LValue);
    let _ = self.visit_ast_expr_value_context(compound_assign.value.get(), ValueContext::RValue);

    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_function(&mut self, stat_fn: &AstStatFunction) -> NonStrictContext {
    // func 已句柄化为 Node（parser 填充同 arena 闭包节点，非空由类型层承载），
    // `get` 直出存活引用。
    self.visit_ast_expr_function(stat_fn.func.get())
  }

  pub(crate) fn visit_ast_stat_local_function(
    &mut self,
    local_fn: &AstStatLocalFunction,
  ) -> NonStrictContext {
    // func 已句柄化为 Node；经 `base` 上转 `&AstExpr`（repr(C) 基址重合，
    // 与原 `cast::<AstExpr>()` 同址）。C++ `visit(localFn->func,
    // ValueContext::RValue)` 经通用 `visit(AstExpr*, ValueContext)` 重载分派
    // （AstExprFunction* 上转）。
    let func_expr: &AstExpr = &local_fn.func.get().base;
    self.visit_ast_expr_value_context(func_expr, ValueContext::RValue)
  }

  pub(crate) fn visit_ast_stat_type_alias(
    &mut self,
    type_alias: &AstStatTypeAlias,
  ) -> NonStrictContext {
    // generics/generic_packs/type_ptr 字段由 parser 填充、随父节点存活。
    self.visit_generics(type_alias.generics, type_alias.generic_packs);
    self.visit_ast_type(alias_opt(type_alias.type_ptr));

    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_declare_function(
    &mut self,
    decl_fn: &AstStatDeclareFunction,
  ) -> NonStrictContext {
    // 声明语句及其 generics/params/ret_types 均在 arena 中。
    self.visit_generics(decl_fn.generics, decl_fn.generic_packs);
    self.visit_ast_type_list(&decl_fn.params);
    self.visit_ast_type_pack(alias_opt(decl_fn.ret_types));

    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_declare_global(
    &mut self,
    decl_global: &AstStatDeclareGlobal,
  ) -> NonStrictContext {
    // type_ 子指针指向 parser 填充的 arena 类型注解。
    self.visit_ast_type(alias_opt(decl_global.type_));
    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_declare_extern_type(
    &mut self,
    decl_class: &AstStatDeclareExternType,
  ) -> NonStrictContext {
    // indexer 为可空槽位（cpp 判空后解引用），句柄 `get` 折叠为 Option；
    // index/result 为恒非空句柄，`get` 直接给出共享引用。
    if let Some(indexer) = decl_class.indexer.get() {
      self.visit_ast_type(Some(indexer.index_type.get()));
      self.visit_ast_type(Some(indexer.result_type.get()));
    }

    for prop in decl_class.props.as_slice() {
      self.visit_ast_type(alias_opt(prop.ty));
    }

    NonStrictContext::new()
  }

  /// cpp `NonStrictTypeChecker::visit(AstStatClass*, ...)`
  /// (NonStrictTypeChecker.cpp:521)：逐成员访问——属性读类型注解、方法向下
  /// 访问函数表达式；`decl_class` 为存活引用，`members` 变体数组由 parser 填充。
  pub fn visit_ast_stat_class(&mut self, decl_class: &AstStatClass) -> NonStrictContext {
    let members = &decl_class.members;
    for prop in members.as_slice() {
      if let Some(property) = prop.get_if_0() {
        self.visit_ast_type(alias_opt(property.ty));
      } else if let Some(method) = prop.get_if_1() {
        // function 为 ast record 未句柄化字段（恒非空，parser 填充同 arena 节点），
        // 经 `alias_ref` 门面（解引用契约集中在 arena_handle）换得存活引用。
        self.visit_ast_expr_function(alias_ref(method.function));
      } else {
        LUAU_ASSERT!(false);
      }
    }

    NonStrictContext::new()
  }

  pub(crate) fn visit_ast_stat_error(&mut self, error: &AstStatError) -> NonStrictContext {
    // statements/expressions 数组元素均由 parser 写入 arena，
    // iter_nodes 安全门面逐个给出引用。
    for stat in error.statements.iter_nodes() {
      self.visit_ast_stat(stat);
    }
    for expr in error.expressions.iter_nodes() {
      self.visit_ast_expr_value_context(expr, ValueContext::RValue);
    }
    NonStrictContext::new()
  }

  /// cpp `visit(AstExpr*, ValueContext)`：`expr` 为 arena 存活且非空的节点
  /// 引用（cpp 侧同一指针直接 `expr->as<T>()`
  /// （NonStrictTypeChecker.cpp:546-604）作相同非空前提）。分派链以
  /// `as_expr_ref` 安全枚举完成，子类下转全部走匹配臂的类型化引用。
  pub fn visit_ast_expr_value_context(
    &mut self,
    expr: &AstExpr,
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

    let node: &AstNode = &expr.base;
    let _pusher = self.push_stack(node);

    match expr.as_expr_ref() {
      AstExprRef::Group(group) => self.visit_ast_expr_group_value_context(group, context),
      // cpp 字面量叶节点与 varargs 的 `visit` 重载均为 `{ return {}; }`
      //（原 visit_ast_expr_constant_nil/bool/number/integer/string、
      // visit_ast_expr_varargs 空体桩并入本臂）。
      AstExprRef::ConstantNil(_)
      | AstExprRef::ConstantBool(_)
      | AstExprRef::ConstantNumber(_)
      | AstExprRef::ConstantInteger(_)
      | AstExprRef::ConstantString(_)
      | AstExprRef::Varargs(_) => NonStrictContext::new(),
      // C++ `visit(AstExprLocal*, ValueContext) { return {}; }`（原
      // visit_ast_expr_local_value_context 空体桩并入本臂）。
      AstExprRef::Local(_) => NonStrictContext::new(),
      AstExprRef::Global(global) => self.visit_ast_expr_global_value_context(global, context),
      AstExprRef::Call(call) => self.visit_ast_expr_call(call),
      AstExprRef::IndexName(index_name) => {
        self.visit_ast_expr_index_name_value_context(index_name, context)
      }
      AstExprRef::IndexExpr(index_expr) => {
        self.visit_ast_expr_index_expr_value_context(index_expr, context)
      }
      AstExprRef::Function(expr_fn) => self.visit_ast_expr_function(expr_fn),
      AstExprRef::Table(table) => self.visit_ast_expr_table(table),
      AstExprRef::Unary(unary) => self.visit_ast_expr_unary(unary),
      AstExprRef::Binary(binary) => self.visit_ast_expr_binary(binary),
      AstExprRef::TypeAssertion(type_assertion) => {
        self.visit_ast_expr_type_assertion(type_assertion)
      }
      AstExprRef::IfElse(if_else) => self.visit_ast_expr_if_else(if_else),
      AstExprRef::InterpString(interp_string) => self.visit_ast_expr_interp_string(interp_string),
      AstExprRef::Error(error) => self.visit_ast_expr_error(error),
      AstExprRef::Instantiate(instantiate) => self.visit_ast_expr_instantiate(instantiate),
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
    // expr 已句柄化：`Node::get` 直出存活只读引用。
    self.visit_ast_expr_value_context(group.expr.get(), context)
  }
}
