use alloc::vec::Vec;

use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_expr_index_name::AstExprIndexName, ast_node},
  rtti::ast_node_try_as,
};

use crate::{
  enums::value_context::ValueContext,
  functions::{
    begin_type_pack::begin_type_pack_id, end_type_pack::end_type_pack_id,
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id,
    get_function_name_as_string::get_function_name_as_string, get_type_alt_j::get_type_id,
    get_type_pack::get_type_pack_id, is_optional::is_optional,
  },
  records::{
    any_type::AnyType as AnyTypeRecord, ast_expr::AstExpr,
    checked_function_call_error::CheckedFunctionCallError,
    checked_function_incorrect_args::CheckedFunctionIncorrectArgs, function_type::FunctionType,
    non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
    type_pack_iterator::TypePackIterator as TypePackIteratorAlias,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    def_id_def::DefId as DefIdAlias, type_error_data::TypeErrorData, type_id::TypeId as TypeIdAlias,
  },
};
impl NonStrictTypeChecker {
  pub fn visit_ast_expr_call(&mut self, call: &AstExprCall) -> NonStrictContext {
    // visit(call->func, ValueContext::RValue);
    let func_ptr = call.func;
    self.visit_ast_expr_value_context(func_ptr, ValueContext::RValue);

    // for (auto arg : call->args) visit(arg, ValueContext::RValue);
    for i in 0..call.args.size {
      // SAFETY: i < args.size，AstArray 元素连续存储。
      let arg = unsafe { *call.args.data.add(i) };
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
    let followed_fn_ty = follow_type_id(fn_ty);
    let Some(fn_ptr) = get_type_id::<FunctionType>(followed_fn_ty) else {
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
        Some(index_expr) => arguments.push(index_expr.expr),
        None => {
          // SAFETY: ice 指向 InternalErrorReporter，存活期覆盖 checker。
          unsafe { (*self.ice).ice_string("method call expression has no 'self'") };
        }
      }
    }

    arguments.extend_from_slice(call.args.as_slice());

    // Collect expected arg types
    let mut arg_types: Vec<TypeIdAlias> = Vec::with_capacity(arguments.len());

    let mut curr: TypePackIteratorAlias = begin_type_pack_id(fn_ptr.arg_types);
    let fin: TypePackIteratorAlias = end_type_pack_id(fn_ptr.arg_types);
    while curr.operator_ne(&fin) {
      let ty: TypeIdAlias = *curr.operator_deref();
      arg_types.push(ty);
      curr.operator_inc();
    }

    if let Some(arg_tail) = curr.tail() {
      let followed = unsafe { follow_type_pack_id(arg_tail) };
      if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(followed) {
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
      // C++ `std::shared_ptr<const NormalizedType> norm = normalizer.normalize(...)`.
      // The landed `normalize` returns an always-present `Arc<NormalizedType>`
      // (never null), so the C++ `if (!norm) reportError(NormalizationTooComplex)`
      // branch is unreachable here.
      let norm = self.normalizer.normalize(expected_arg_type);

      let any_ptr = get_type_id::<AnyTypeRecord>(norm.tops);
      let run_time_error_ty: TypeIdAlias = if any_ptr.is_some() {
        // SAFETY: builtin_types 在 checker 存活期内有效。
        unsafe { (*self.builtin_types).never_type }
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
}
