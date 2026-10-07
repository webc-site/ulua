use alloc::vec::Vec;
use core::ptr::null;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_index_name::AstExprIndexName,
    ast_node::AstNode, location::Location,
  },
  rtti::ast_node_try_as,
};
use ulua_common::fflag;

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id, flatten_intersection::flatten_intersection, follow_type,
    get_mutable_type_pack, get_type, get_type_pack, shared_mut::shared_mut,
  },
  methods::type_checker_check_call_overload::CheckCallOverloadArgs,
  records::{
    arena_handle::{alias, alias_opt, alias_ref},
    free_type::FreeType,
    function_type::FunctionType,
    overload_error_entry::OverloadErrorEntry,
    type_checker::TypeChecker,
    type_pack::TypePack,
    type_pack_var::TypePackVar,
    with_predicate::WithPredicate,
  },
  type_aliases::{
    error_type_pack::ErrorTypePack, name_type::Name, scope_ptr_type::ScopePtr, type_id::TypeId,
    type_pack_id::TypePackId, type_variant::TypeVariant,
  },
};

impl TypeChecker {
  pub fn check_expr_pack_helper_scope_ptr_ast_expr(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExpr,
  ) -> WithPredicate<TypePackId> {
    match expr.as_expr_ref() {
      AstExprRef::Call(call_expr) => {
        self.check_expr_pack_helper_scope_ptr_ast_expr_call(scope, call_expr)
      }
      AstExprRef::Varargs(_) => {
        let tp = match scope.vararg_pack {
          Some(pack) => pack,
          None => self.error_recovery_type_pack_scope_ptr(scope.clone()),
        };
        WithPredicate::with_predicate_t(tp)
      }
      _ => {
        let type_result = self.check_expr(scope, expr, None, false);
        WithPredicate::with_predicate_t(
          self.add_type_pack_type_pack_var(TypePackVar::from(TypePack::single(type_result.r#type))),
        )
      }
    }
  }

  fn check_expr_pack_helper_scope_ptr_ast_expr_call(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprCall,
  ) -> WithPredicate<TypePackId> {
    // evaluate type of function
    // decompose an intersection into its component overloads
    // Compute type_arguments of parameters
    // For each overload
    //     Compare parameter and argument type_arguments
    //     Report any errors (also speculate dot vs colon warnings!)
    //     Return the resulting return type (even if there are errors)
    // If there are no matching overloads, unify with (a...) -> (b...) and return b...

    let func_loc = alias_ref(expr.func).base.location;

    // §2 清扫批（W3）判定：业务层未初始化占位哨兵消解为 `Option`——cpp
    // TypeInfer.cpp:4523 `TypeId selfType = nullptr;` 的空值仅在 `expr.self` 为假时存在，
    // 且下游只在 `if (expr.self)` 守卫内读取（cpp/Rust 同条件），空值从不被消费，不属
    // cpp 侧「未命中/缺席」语义。缺席分支（非 self 调用不前置首参）由 `Option::None` +
    // `if let` 逐字保留，未吞掉任何守卫。
    let (function_type, actual_function_type, self_type) = if expr.self_ {
      let Some(index_expr) =
        ast_node_try_as::<AstExprIndexName>(alias_ref(expr.func as *const AstNode))
      else {
        self.ice_string("method call expression has no 'self'");
        return WithPredicate::with_predicate_t(
          self.error_recovery_type_pack_scope_ptr(scope.clone()),
        );
      };

      let self_type = self
        .check_expr(
          scope,
          // index_expr.expr 已句柄化恒非空：.get() 共享引用仅供本次只读 checkExpr。
          index_expr.expr.get(),
          None,
          false,
        )
        .r#type;
      let self_type = self.strip_from_nil_and_report(self_type, &func_loc);

      // Note: index 是 parser 拷入 arena 的 NUL 结尾 C 字符串，as_str_or_empty
      // 只读共享借用，不涉及 unsafe。
      let index_name: Name = index_expr.index.as_str_or_empty().to_string();
      let prop_ty = self.get_index_type_from_type(
        scope.clone(),
        self_type,
        &index_name,
        &expr.base.base.location,
        /* addErrors= */ true,
      );
      let (function_type, actual_function_type) = if let Some(prop_ty) = prop_ty {
        let function_type = prop_ty;
        let to_instantiate =
          if fflag::LuauExplicitTypeInstantiationSupport.get() && expr.type_arguments.size != 0 {
            self.instantiate_type_parameters(
              scope.clone(),
              function_type,
              expr.type_arguments,
              alias_opt(expr.func),
              &expr.base.base.location,
            )
          } else {
            function_type
          };
        (
          function_type,
          self.instantiate(scope, to_instantiate, func_loc, null()),
        )
      } else {
        let function_type = self.error_recovery_type_scope_ptr(scope);
        (function_type, function_type)
      };
      (function_type, actual_function_type, Some(self_type))
    } else {
      let function_type = self
        .check_expr(scope, alias_ref(expr.func), None, false)
        .r#type;
      (
        function_type,
        self.instantiate(scope, function_type, func_loc, null()),
        None,
      )
    };

    let ret_pack: TypePackId;
    if let Some(free) = get_type::get::<FreeType>(actual_function_type) {
      ret_pack = self.fresh_type_pack_type_level(free.level);
      let fresh_arg_pack = self.fresh_type_pack_type_level(free.level);
      let level = free.level;
      let mut function = FunctionType::function_type_new(fresh_arg_pack, ret_pack, None, false);
      function.level = level;
      alias(as_mutable_type_id(actual_function_type)).ty = TypeVariant::Function(function);
    } else {
      ret_pack = self.fresh_type_pack_type_level(scope.level);
    }

    // We break this function up into a lambda here to limit our stack footprint.
    // The vectors used by this function aren't allocated until the lambda is actually called.

    // checkExpr will log the pre-instantiated type of the function.
    // That's not nearly as interesting as the instantiated type, which will include details about how
    // generic functions are being instantiated for this particular callsite.
    {
      // Safety: `shared_mut` 惯用法——current_module 由检查驱动在表达式检查
      // 开始前注入并在整模块检查期间持有（unwrap 的 Some 前提同 C++
      // `TypeChecker::currentModule` 恒非空）；单线程检查期内仅本函数经此
      // 裸句柄写 Module 的 astOriginalCallTypes/astTypes 两张映射，`&mut`
      // 借出被本块包裹、块尾即归还，无并存别名。语义对应 C++
      // TypeInfer.cpp:4574 `currentModule->astOriginalCallTypes[expr.func]`。
      let module = { &mut *(shared_mut(self.expect_current_module())) };
      *module
        .ast_original_call_types
        .get_or_insert(expr.func as *const AstNode) = follow_type::follow(function_type);
      *module.ast_types.get_or_insert(expr.func as *const AstExpr) = actual_function_type;
    }

    let overloads = flatten_intersection(actual_function_type);

    let expected_types = self.get_expected_types_for_call(&overloads, expr.args.size, expr.self_);

    let mut arg_list_result = self.check_expr_list(
      scope,
      &expr.base.base.location,
      &expr.args,
      false,
      &Vec::new(),
      &expected_types,
    );
    let mut arg_pack = arg_list_result.r#type;

    if get_type_pack::get::<ErrorTypePack>(arg_pack).is_some() {
      return WithPredicate::with_predicate_t(
        self.error_recovery_type_pack_scope_ptr(scope.clone()),
      );
    }

    // 原 `if expr.self_` + 可空 `selfType` 哨兵的 cpp 形态：self_type 仅在 self 调用时为
    // `Some`，`if let` 守卫与 cpp 缺席分支逐字同构。
    if let Some(self_type) = self_type {
      arg_pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack::new(
        Vec::from([self_type]),
        Some(arg_pack),
      )));
      arg_list_result.r#type = arg_pack;
    }
    // C++ 对照 TypeInfer.cpp:4536 `LUAU_ASSERT(args)`：argPack 由 checkExprList /
    // addTypePack(TypePack) 产生，必为 TypePack 变体，expect 不会触发。
    let args =
      get_mutable_type_pack::get_mutable::<TypePack>(arg_pack).expect("argPack is a TypePack");

    let mut arg_locations: Vec<Location> = Vec::with_capacity(expr.args.size + 1);
    if expr.self_ {
      let index_expr = ast_node_try_as::<AstExprIndexName>(alias_ref(expr.func as *const AstNode))
        .expect("同一 func 指针上文 try_as 已命中（失败支已提前返回），见上注释");
      // index_expr.expr 已句柄化恒非空；此处只拷贝其 base.location（Copy 字段）。
      arg_locations.push(index_expr.expr.get().base.location);
    }
    // iter_nodes 收口逐元素解引用的 unsafe：args 元素由 parser 成对写入
    // arena（判过 size），只读取每个表达式的 location。
    for arg in expr.args.iter_nodes() {
      arg_locations.push(arg.base.location);
    }

    let mut errors: Vec<OverloadErrorEntry> = Vec::new(); // errors encountered for each overload

    let mut overloads_that_match_arg_count: Vec<TypeId> = Vec::new();
    let mut overloads_that_dont: Vec<TypeId> = Vec::new();

    for fn_ty in overloads.iter() {
      let fn_ty = follow_type::follow(*fn_ty);

      if let Some(ret) = self.check_call_overload(CheckCallOverloadArgs {
        scope,
        expr,
        fn_ty,
        ret_pack,
        arg_pack,
        args,
        arg_locations: &arg_locations,
        arg_list_result: &arg_list_result,
        overloads_that_match_arg_count: &mut overloads_that_match_arg_count,
        overloads_that_dont: &mut overloads_that_dont,
        errors: &mut errors,
      }) {
        return *ret;
      }
    }

    if self.handle_self_call_mismatch(scope, expr, args, &arg_locations, &errors) {
      return WithPredicate::with_predicate_t(ret_pack);
    }

    self.report_overload_resolution_error(
      scope,
      expr,
      ret_pack,
      arg_pack,
      &arg_locations,
      &overloads,
      &overloads_that_match_arg_count,
      &mut errors,
    );

    let mut overload: Option<&FunctionType> = None;
    if !overloads_that_match_arg_count.is_empty() {
      overload = get_type::get::<FunctionType>(overloads_that_match_arg_count[0]);
    }
    if overload.is_none() && !overloads_that_dont.is_empty() {
      overload = get_type::get::<FunctionType>(overloads_that_dont[0]);
    }
    if let Some(overload) = overload {
      return WithPredicate::with_predicate_t(
        self.error_recovery_type_pack_type_pack_id(overload.ret_types),
      );
    }

    WithPredicate::with_predicate_t(self.error_recovery_type_pack_type_pack_id(ret_pack))
  }
}
