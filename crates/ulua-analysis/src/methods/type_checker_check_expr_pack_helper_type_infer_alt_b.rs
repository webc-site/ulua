use alloc::{sync::Arc, vec::Vec};
use core::{
  ffi::CStr,
  ptr::{null, null_mut},
};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_index_name::AstExprIndexName,
    ast_node::AstNode, location::Location,
  },
  rtti::ast_node_try_as,
};
use ulua_common::FFlag;

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id, flatten_intersection::flatten_intersection,
    follow_type::follow_type_id, get_mutable_type_pack::get_mutable_type_pack_id,
    get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id,
  },
  methods::type_checker_check_call_overload::CheckCallOverloadArgs,
  records::{
    free_type::FreeType, function_type::FunctionType, module::Module,
    overload_error_entry::OverloadErrorEntry, type_checker::TypeChecker, type_pack::TypePack,
    type_pack_var::TypePackVar, with_predicate::WithPredicate,
  },
  type_aliases::{
    error_type_pack::ErrorTypePack, name_type::Name, scope_ptr_type::ScopePtr, type_id::TypeId,
    type_pack_id::TypePackId, type_variant::TypeVariant,
  },
};
impl TypeChecker {
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

    // SAFETY: expr.func 指向 AST arena 节点（parser 保证非空）。
    let func_loc = unsafe { (*expr.func).base.location };

    let mut self_type: TypeId = null_mut();
    let function_type: TypeId;
    let actual_function_type: TypeId;

    if expr.self_ {
      // SAFETY: AstExpr #[repr(C)] 单继承，base(AstNode) 在偏移 0，cast 有效。
      let Some(index_expr) =
        ast_node_try_as::<AstExprIndexName>(unsafe { &*(expr.func as *const AstNode) })
      else {
        self.ice_string("method call expression has no 'self'");
        return WithPredicate::with_predicate_t(
          self.error_recovery_type_pack_scope_ptr(scope.clone()),
        );
      };

      self_type = self
        .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
          scope,
          // SAFETY: index_expr->expr 指向 AST arena 节点。
          unsafe { &*index_expr.expr },
          None,
          false,
        )
        .r#type;
      self_type = self.strip_from_nil_and_report(self_type, &func_loc);

      // SAFETY: index.value 为 NUL 结尾 C 字符串（AST arena 持有）。
      let index_name: Name = unsafe {
        CStr::from_ptr(index_expr.index.value)
          .to_string_lossy()
          .into_owned()
      };
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
          if FFlag::LuauExplicitTypeInstantiationSupport.get() && expr.type_arguments.size != 0 {
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
        .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
          scope,
          // SAFETY: expr.func 指向 AST arena 节点。
          unsafe { &*expr.func },
          None,
          false,
        )
        .r#type;
      actual_function_type = self.instantiate(scope, function_type, func_loc, null());
    }

    let ret_pack: TypePackId;
    if let Some(free) = get_type_id::<FreeType>(actual_function_type) {
      ret_pack = self.fresh_type_pack_type_level(free.level);
      let fresh_arg_pack = self.fresh_type_pack_type_level(free.level);
      let level = free.level;
      let mut function = FunctionType::function_type_new(fresh_arg_pack, ret_pack, None, false);
      function.level = level;
      // SAFETY: as_mutable_type_id 去除 const（C++ asMutable 同义），句柄有效。
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
      // SAFETY: current_module 在类型检查期间独占使用（C++ 直接改
      // module->astOriginalCallTypes / astTypes 同义）。
      let module =
        unsafe { &mut *(Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module) };
      *module
        .ast_original_call_types
        .get_or_insert(expr.func as *const AstNode) = follow_type_id(function_type);
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

    if get_type_pack_id::<ErrorTypePack>(arg_pack).is_some() {
      return WithPredicate::with_predicate_t(
        self.error_recovery_type_pack_scope_ptr(scope.clone()),
      );
    }

    if expr.self_ {
      arg_pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
        head: Vec::from([self_type]),
        tail: Some(arg_pack),
      }));
      arg_list_result.r#type = arg_pack;
    }
    // C++ 对照 TypeInfer.cpp:4536 `LUAU_ASSERT(args)`：argPack 由 checkExprList /
    // addTypePack(TypePack) 产生，必为 TypePack 变体，expect 不会触发。
    let args = get_mutable_type_pack_id::<TypePack>(arg_pack).expect("argPack is a TypePack");

    let mut arg_locations: Vec<Location> = Vec::with_capacity(expr.args.size + 1);
    if expr.self_ {
      // SAFETY: 同上文下转，self_ 调用的 func 必为 AstExprIndexName。
      let index_expr =
        ast_node_try_as::<AstExprIndexName>(unsafe { &*(expr.func as *const AstNode) }).unwrap();
      // SAFETY: index_expr->expr 指向 AST arena 节点。
      arg_locations.push(unsafe { (*index_expr.expr).base.location });
    }
    for arg in expr.args.iter() {
      // SAFETY: arg 指向 AST arena 节点。
      arg_locations.push(unsafe { (**arg).base.location });
    }

    let mut errors: Vec<OverloadErrorEntry> = Vec::new(); // errors encountered for each overload

    let mut overloads_that_match_arg_count: Vec<TypeId> = Vec::new();
    let mut overloads_that_dont: Vec<TypeId> = Vec::new();

    for fn_ty in overloads.iter() {
      let fn_ty = follow_type_id(*fn_ty);

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
      overload = get_type_id::<FunctionType>(overloads_that_match_arg_count[0]);
    }
    if overload.is_none() && !overloads_that_dont.is_empty() {
      overload = get_type_id::<FunctionType>(overloads_that_dont[0]);
    }
    if let Some(overload) = overload {
      return WithPredicate::with_predicate_t(
        self.error_recovery_type_pack_type_pack_id(overload.ret_types),
      );
    }

    WithPredicate::with_predicate_t(self.error_recovery_type_pack_type_pack_id(ret_pack))
  }
}
