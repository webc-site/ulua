use alloc::vec::Vec;
use core::ptr::{NonNull, null_mut};
use std::{cmp::min, sync::Arc};

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
    ast_expr_group::AstExprGroup, ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::{
  functions::{flatten_type_pack::flatten_type_pack_id, parse_format_string::parse_format_string},
  records::{
    count_mismatch::{CountMismatch, CountMismatchContext},
    module::Module,
    type_checker::TypeChecker,
    type_error::TypeError,
    type_error_data::TypeErrorData,
    with_predicate::WithPredicate,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId},
};
pub fn magic_format_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &ScopePtr,
  expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let (param_pack, _predicates) = (with_predicate.r#type, with_predicate.predicates);

  let module = typechecker.current_module.as_ref()?;
  let arena = unsafe { &mut (*(Arc::as_ptr(module) as *mut Module)).internal_types };

  let mut fmt: *mut AstExprConstantString = null_mut();

  if expr.self_ {
    let index = unsafe { ast_node_as::<AstExprIndexName>(expr.func as *mut AstNode) };
    if !index.is_null() {
      let group = unsafe { ast_node_as::<AstExprGroup>((*index).expr as *mut AstNode) };
      if !group.is_null() {
        fmt = unsafe { ast_node_as::<AstExprConstantString>((*group).expr as *mut AstNode) };
      } else {
        fmt = unsafe { ast_node_as::<AstExprConstantString>((*index).expr as *mut AstNode) };
      }
    }
  }

  if !expr.self_ && expr.args.size > 0 {
    fmt =
      unsafe { ast_node_as::<AstExprConstantString>({ *expr.args.data.add(0) } as *mut AstNode) };
  }

  if fmt.is_null() {
    return None;
  }

  let expected: Vec<TypeId> = unsafe {
    parse_format_string(
      NonNull::new_unchecked(typechecker.builtin_types),
      (*fmt).value.data,
      (*fmt).value.size,
    )
  };

  let (params, tail) = flatten_type_pack_id(param_pack);

  let param_offset: usize = 1;
  let data_offset: usize = if expr.self_ { 0 } else { 1 };

  for (i, &expected_ty) in expected.iter().enumerate() {
    let Some(param) = params.get(i + param_offset) else {
      break;
    };
    // No argument expressions ⇒ nothing to attach a location to, and
    // `args.size - 1` would underflow (the self-call path lacks the
    // `args.size > 0` guard the non-self path has).
    if expr.args.size == 0 {
      break;
    }

    let arg_index = min(i + data_offset, expr.args.size - 1);
    let location = unsafe { &(*(*expr.args.data.add(arg_index as usize))).base.location };

    typechecker.unify_type_id_type_id_scope_ptr_location(*param, expected_ty, scope, location);
  }

  let num_actual_params = params.len();
  let num_expected_params = expected.len() + 1;

  if num_expected_params != num_actual_params
    && (!tail.is_some() || num_expected_params < num_actual_params)
  {
    let error = TypeError::type_error_location_type_error_data(
      expr.base.base.location,
      TypeErrorData::CountMismatch(CountMismatch {
        expected: num_expected_params,
        maximum: None,
        actual: num_actual_params,
        context: CountMismatchContext::Arg,
        is_variadic: false,
        function: String::new(),
      }),
    );
    typechecker.report_error_type_error(&error);
  }

  Some(WithPredicate::with_predicate_t(
    arena.add_type_pack_initializer_list_type_id(&[typechecker.string_type]),
  ))
}
