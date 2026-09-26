use alloc::vec::Vec;
use core::ptr::{null, null_mut};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_index_name::AstExprIndexName,
    location::Location,
  },
  rtti::{ast_node_try_as, ast_rtti_index},
};
use ulua_common::fflag;

use crate::{
  functions::{
    arc_as_mut::arc_as_mut, as_mutable_type::as_mutable_type_id,
    flatten_intersection::flatten_intersection, follow_type, get_mutable_type_pack, get_type,
    get_type_pack,
  },
  methods::type_checker_check_call_overload::CheckCallOverloadArgs,
  records::{
    ast_node::AstNode, free_type::FreeType, function_type::FunctionType,
    overload_error_entry::OverloadErrorEntry, type_checker::TypeChecker, type_pack::TypePack,
    type_pack_var::TypePackVar, with_predicate::WithPredicate,
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
    // 安全等价改写：原先 `expr as *mut AstExpr` + 裸指针解引用只读取 base
    // (AstNode) 字段与下转，全部经共享引用完成，与 C++
    // `expr.as<AstExprCall>()` / `expr.is<AstExprVarargs>()` 同构。
    // ast_rtti_index("AstExprCall") 即 AstExprCall::CLASS_INDEX 的取值来源，
    // 上一行判定命中则 try_as 必为 Some，unwrap 无 panic 路径。
    if expr.base.class_index == ast_rtti_index("AstExprCall") {
      let call_expr = ast_node_try_as::<AstExprCall>(&expr.base)
        .expect("上一行 class_index 判据与 try_as 同构，必为 Some（见上注释）");
      self.check_expr_pack_helper_scope_ptr_ast_expr_call(scope, call_expr)
    } else if expr.base.class_index == ast_rtti_index("AstExprVarargs") {
      if scope.vararg_pack.is_none() {
        return WithPredicate::with_predicate_t(
          self.error_recovery_type_pack_scope_ptr(scope.clone()),
        );
      }
      WithPredicate::with_predicate_t(
        scope
          .vararg_pack
          .expect("上一行 is_none() 分支已早返，此处必为 Some"),
      )
    } else {
      let type_result = self.check_expr(scope, expr, None, false);
      WithPredicate::with_predicate_t(
        self.add_type_pack_type_pack_var(TypePackVar::from(TypePack::single(type_result.r#type))),
      )
    }
  }

  pub fn check_expr_pack_helper_scope_ptr_ast_expr_call(
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

    // Safety: expr.func 是 parser 为 AstExprCall 写入的非空被调表达式指针，
    // 指向 SourceModule 持有的 AST arena 节点，随整个检查期存活；本次解
    // 引用只拷贝 base.location（Location 是 Copy 值），借用不超出语句。对
    // 应 C++ `Location funcLocation = expr.func->location`。
    let func_loc = unsafe { (*expr.func).base.location };

    let mut self_type: TypeId = null_mut();
    let function_type: TypeId;
    let actual_function_type: TypeId;

    if expr.self_ {
      // Safety: AstExpr #[repr(C)] 单继承，首字段 base(AstNode) 与整节点同址，
      // 故 `expr.func as *const AstNode` 重解释合法；expr.func 是 parser 写入
      // 的非空 arena 节点。try_as 只读 class_index 并在命中时返回共享引用，
      // 未命中走 ice 提前返回，与 C++ `expr.func.as<AstExprIndexName>()` 对齐。
      let Some(index_expr) =
        ast_node_try_as::<AstExprIndexName>(unsafe { &*(expr.func as *const AstNode) })
      else {
        self.ice_string("method call expression has no 'self'");
        return WithPredicate::with_predicate_t(
          self.error_recovery_type_pack_scope_ptr(scope.clone()),
        );
      };

      self_type = self
        .check_expr(
          scope,
          // index_expr.expr 已句柄化恒非空：.get() 共享引用仅供本次只读 checkExpr。
          index_expr.expr.get(),
          None,
          false,
        )
        .r#type;
      self_type = self.strip_from_nil_and_report(self_type, &func_loc);

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
      if let Some(prop_ty) = prop_ty {
        function_type = prop_ty;
        let to_instantiate =
          if fflag::LuauExplicitTypeInstantiationSupport.get() && expr.type_arguments.size != 0 {
            // Safety: 被调 `instantiate_type_parameters` 契约要求——
            // `function_type` 是刚由 getIndexedType 产出的 arena 存活 TypeId；
            // `expr.type_arguments` 各元素的 type/type_pack 槽为 parser 写入的
            // arena 注解（被调方内部逐元素判空）；`expr.func` 为非空 AST
            // 节点指针仅用于报错定位。借用均止于本次调用。
            unsafe {
              self.instantiate_type_parameters(
                scope.clone(),
                function_type,
                expr.type_arguments,
                expr.func as *const AstExpr,
                &expr.base.base.location,
              )
            }
          } else {
            function_type
          };
        actual_function_type = self.instantiate(scope, to_instantiate, func_loc, null());
      } else {
        function_type = self.error_recovery_type_scope_ptr(scope);
        actual_function_type = function_type;
      }
    } else {
      function_type = self
        .check_expr(
          scope,
          // Safety: expr.func 同 func_loc 处论证——parser 写入的非空 arena
          // 被调表达式指针；本次借出的共享引用仅供只读 checkExpr，随调用
          // 结束，不与他处借用重叠。
          unsafe { &*expr.func },
          None,
          false,
        )
        .r#type;
      actual_function_type = self.instantiate(scope, function_type, func_loc, null());
    }

    let ret_pack: TypePackId;
    if let Some(free) = get_type::get::<FreeType>(actual_function_type) {
      ret_pack = self.fresh_type_pack_type_level(free.level);
      let fresh_arg_pack = self.fresh_type_pack_type_level(free.level);
      let level = free.level;
      let mut function = FunctionType::function_type_new(fresh_arg_pack, ret_pack, None, false);
      function.level = level;
      // Safety: as_mutable_type_id 与 C++ `asMutable` 同义（TypeId 去 const
      // 得同址 *mut Type，节点在推理 arena 中地址稳定）。actual_function_type
      // 刚由 instantiate 产出且确认是 FreeType 变体；上方对 free 的共享读在
      // `level` 取出后即结束，此刻写穿 `.ty` 无并存借用，对应 C++
      // TypeInfer.cpp:4562 `emplaceType<FunctionType>(asMutable(...))`。
      unsafe {
        (*as_mutable_type_id(actual_function_type)).ty = TypeVariant::Function(function);
      }
    } else {
      ret_pack = self.fresh_type_pack_type_level(scope.level);
    }

    // We break this function up into a lambda here to limit our stack footprint.
    // The vectors used by this function aren't allocated until the lambda is actually called.

    // checkExpr will log the pre-instantiated type of the function.
    // That's not nearly as interesting as the instantiated type, which will include details about how
    // generic functions are being instantiated for this particular callsite.
    {
      // Safety: `arc_as_mut` 惯用法——current_module 由检查驱动在表达式检查
      // 开始前注入并在整模块检查期间持有（unwrap 的 Some 前提同 C++
      // `TypeChecker::currentModule` 恒非空）；单线程检查期内仅本函数经此
      // 裸句柄写 Module 的 astOriginalCallTypes/astTypes 两张映射，`&mut`
      // 借出被本块包裹、块尾即归还，无并存别名。语义对应 C++
      // TypeInfer.cpp:4574 `currentModule->astOriginalCallTypes[expr.func]`。
      let module = unsafe {
        &mut *(arc_as_mut(
          self.current_module.as_ref().expect(
            "current_module 由 check_without_recursion_check 入口置入 Some、末尾才 take()，check 调用树内恒为 Some",
          ),
        ))
      };
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

    if expr.self_ {
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
      // Safety: 同一 expr.func 的非空 arena 节点指针 + repr(C) 基址重合下转；
      // self_ 路径上文已对同一指针 try_as 成功（失败分支已提前返回），故
      // 此处必为 Some，unwrap 无 panic 路径。
      let index_expr =
        ast_node_try_as::<AstExprIndexName>(unsafe { &*(expr.func as *const AstNode) })
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
