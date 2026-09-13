use alloc::vec;
use core::ptr::{NonNull, null_mut};

use ulua_ast::{
  records::{
    ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_string::AstExprConstantString,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::{
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
    parse_pattern_string::parse_pattern_string_bytes,
  },
  records::{
    magic_find::MagicFind, magic_function_call_context::MagicFunctionCallContext,
    union_type::UnionType,
  },
  type_aliases::{type_id::TypeId, type_pack_variant::TypePackVariant},
};
impl MagicFind {
  pub fn infer(&self, context: &MagicFunctionCallContext) -> bool {
    let (params, _tail) = flatten_type_pack_id(context.arguments);

    if params.len() < 2 || params.len() > 4 {
      return false;
    }

    let solver = unsafe { context.solver.as_ref() };
    let arena = unsafe { &mut *solver.arena };
    let builtin_types = unsafe { &*solver.builtin_types };
    let call_site = unsafe { context.call_site.as_ref() };

    let pattern_index = if call_site.self_ { 0 } else { 1 };
    let pattern = if call_site.args.size > pattern_index {
      let expr = unsafe { *call_site.args.data.add(pattern_index) };
      unsafe { ast_node_as::<AstExprConstantString>(expr as *mut AstNode) }
    } else {
      null_mut()
    };

    if pattern.is_null() {
      return false;
    }

    let mut plain = false;
    let plain_index = if call_site.self_ { 2 } else { 3 };
    if call_site.args.size > plain_index {
      let expr = unsafe { *call_site.args.data.add(plain_index) };
      let bool_expr = unsafe { ast_node_as::<AstExprConstantBool>(expr as *mut AstNode) };
      if !bool_expr.is_null() {
        plain = unsafe { &*bool_expr }.value;
      }
    }

    let mut return_types: Vec<TypeId> = Vec::new();
    if !plain {
      return_types = unsafe {
        parse_pattern_string_bytes(
          NonNull::new_unchecked(solver.builtin_types),
          (*pattern).value.as_bytes(),
        )
      };

      if return_types.is_empty() {
        return false;
      }
    }

    unsafe {
      (*context.solver.as_ptr()).constraint_solver_unify(
        context.constraint.as_ptr(),
        params[0],
        builtin_types.string_type,
      );
    }

    let optional_number = arena.add_type(UnionType {
      options: vec![builtin_types.nil_type, builtin_types.number_type],
    });
    let optional_boolean = arena.add_type(UnionType {
      options: vec![builtin_types.nil_type, builtin_types.boolean_type],
    });

    let init_index = if call_site.self_ { 1 } else { 2 };
    if params.len() >= 3 && call_site.args.size > init_index {
      unsafe {
        (*context.solver.as_ptr()).constraint_solver_unify(
          context.constraint.as_ptr(),
          params[2],
          optional_number,
        );
      }
    }

    if params.len() == 4 && call_site.args.size > plain_index {
      unsafe {
        (*context.solver.as_ptr()).constraint_solver_unify(
          context.constraint.as_ptr(),
          params[3],
          optional_boolean,
        );
      }
    }

    return_types.insert(0, optional_number);
    return_types.insert(1, optional_number);

    let return_list = arena.add_type_pack_vector_type_id_optional_type_pack_id(return_types, None);
    let result_mut = as_mutable_type_pack(context.result);
    unsafe {
      (*result_mut).ty = TypePackVariant::Bound(return_list);
    }

    true
  }
}
