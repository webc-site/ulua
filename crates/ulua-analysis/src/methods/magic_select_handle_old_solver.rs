use core::ffi::c_char;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::{
  functions::flatten_type_pack::flatten_type_pack_id,
  records::{
    generic_error::GenericError, type_checker::TypeChecker, type_error::TypeError,
    with_predicate::WithPredicate,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
pub fn magic_select_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &ScopePtr,
  expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let (param_pack, _predicates) = (with_predicate.r#type, with_predicate.predicates);

  let _ = scope;

  if expr.args.size == 0 {
    typechecker.report_error_type_error(&TypeError::type_error_location_type_error_data(
      expr.base.base.location,
      TypeErrorData::GenericError(GenericError::new(
        "select should take 1 or more arguments".to_string(),
      )),
    ));
    return None;
  }

  let arg1 = unsafe { *expr.args.data.add(0) };
  let num = unsafe { ast_node_as::<AstExprConstantNumber>(arg1 as *mut AstNode) };
  if !num.is_null() {
    let (params, tail) = flatten_type_pack_id(param_pack);

    let offset = unsafe { (*num).value } as i32;
    if offset > 0 {
      let offset_usize = offset as usize;
      if offset_usize < params.len() {
        let result: Vec<TypeId> = params[offset_usize..].to_vec();
        return Some(WithPredicate::with_predicate_t(
          typechecker.add_type_pack_vector_type_id_optional_type_pack_id(&result, tail),
        ));
      } else if tail.is_some() {
        return Some(WithPredicate::with_predicate_t(tail.unwrap()));
      }
    }

    typechecker.report_error_type_error(&TypeError::type_error_location_type_error_data(
      unsafe { (*num).base.base.location },
      TypeErrorData::GenericError(GenericError::new(
        "bad argument #1 to select (index out of range)".to_string(),
      )),
    ));
  } else {
    let str_expr = unsafe { ast_node_as::<AstExprConstantString>(arg1 as *mut AstNode) };
    if !str_expr.is_null()
      && unsafe { (*str_expr).value.size } == 1
      && unsafe { *(*str_expr).value.data } == b'#' as c_char
    {
      return Some(WithPredicate::with_predicate_t(
        typechecker.add_type_pack_initializer_list_type_id(&[typechecker.number_type]),
      ));
    }
  }

  None
}
