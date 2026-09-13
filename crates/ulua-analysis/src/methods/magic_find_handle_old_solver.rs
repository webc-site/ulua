use alloc::{sync::Arc, vec, vec::Vec};
use core::ptr::{NonNull, null_mut};

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_string::AstExprConstantString, ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::{
  functions::{
    flatten_type_pack::flatten_type_pack_id, parse_pattern_string::parse_pattern_string_bytes,
  },
  records::{
    module::Module, type_checker::TypeChecker, type_pack::TypePack, union_type::UnionType,
    with_predicate::WithPredicate,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId},
};
pub fn magic_find_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &ScopePtr,
  expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let param_pack = with_predicate.r#type;
  let (params, _tail) = flatten_type_pack_id(param_pack);

  if params.len() < 2 || params.len() > 4 {
    return None;
  }

  let module = typechecker.current_module.as_ref()?;
  let arena = unsafe { &mut (*(Arc::as_ptr(module) as *mut Module)).internal_types };

  let pattern_index = if expr.self_ { 0 } else { 1 };
  let pattern = if expr.args.size > pattern_index {
    let arg = unsafe { *expr.args.data.add(pattern_index) };
    unsafe { ast_node_as::<AstExprConstantString>(arg as *mut AstNode) }
  } else {
    null_mut()
  };

  if pattern.is_null() {
    return None;
  }

  let plain_index = if expr.self_ { 2 } else { 3 };
  let mut plain = false;
  if expr.args.size > plain_index {
    let arg = unsafe { *expr.args.data.add(plain_index) };
    let p = unsafe { ast_node_as::<AstExprConstantBool>(arg as *mut AstNode) };
    plain = !p.is_null() && unsafe { (*p).value };
  }

  let mut return_types: Vec<TypeId> = Vec::new();
  if !plain {
    return_types = unsafe {
      parse_pattern_string_bytes(
        NonNull::new_unchecked(typechecker.builtin_types),
        (*pattern).value.as_bytes(),
      )
    };

    if return_types.is_empty() {
      return None;
    }
  }

  let first_location = unsafe { &(*(*expr.args.data.add(0))).base.location };
  typechecker.unify_type_id_type_id_scope_ptr_location(
    params[0],
    typechecker.string_type,
    scope,
    first_location,
  );

  let optional_number = arena.add_type(UnionType {
    options: vec![typechecker.nil_type, typechecker.number_type],
  });
  let optional_boolean = arena.add_type(UnionType {
    options: vec![typechecker.nil_type, typechecker.boolean_type],
  });

  let init_index = if expr.self_ { 1 } else { 2 };
  if params.len() >= 3 && expr.args.size > init_index {
    let location = unsafe { &(*(*expr.args.data.add(init_index))).base.location };
    typechecker.unify_type_id_type_id_scope_ptr_location(
      params[2],
      optional_number,
      scope,
      location,
    );
  }

  if params.len() == 4 && expr.args.size > plain_index {
    let location = unsafe { &(*(*expr.args.data.add(plain_index))).base.location };
    typechecker.unify_type_id_type_id_scope_ptr_location(
      params[3],
      optional_boolean,
      scope,
      location,
    );
  }

  return_types.insert(0, optional_number);
  return_types.insert(1, optional_number);

  let return_list = arena.add_type_pack_t(TypePack {
    head: return_types,
    tail: None,
  });
  Some(WithPredicate::with_predicate_t(return_list))
}
