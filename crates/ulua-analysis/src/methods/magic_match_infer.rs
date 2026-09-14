use alloc::vec;
use core::ptr::{NonNull, null_mut};

use ulua_ast::{
  records::{ast_expr_constant_string::AstExprConstantString, ast_node::AstNode},
  rtti::ast_node_as,
};

use crate::{
  functions::{
    as_mutable_type_pack_alt_d::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
    parse_pattern_string::parse_pattern_string_bytes,
  },
  records::{
    magic_function_call_context::MagicFunctionCallContext, type_pack::TypePack,
    union_type::UnionType,
  },
  type_aliases::type_pack_variant::TypePackVariant,
};
pub fn magic_match_infer(context: &MagicFunctionCallContext) -> bool {
  let (params, _tail) = flatten_type_pack_id(context.arguments);

  if params.len() < 2 || params.len() > 3 {
    return false;
  }

  let solver = unsafe { context.solver.as_ref() };
  let arena = unsafe { &mut *solver.arena };
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

  let return_types = unsafe {
    let pattern = &*pattern;
    parse_pattern_string_bytes(
      NonNull::new_unchecked(solver.builtin_types),
      pattern.value.as_bytes(),
    )
  };

  if return_types.is_empty() {
    return false;
  }

  unsafe {
    let builtin_types = &*solver.builtin_types;
    (*context.solver.as_ptr()).constraint_solver_unify(
      context.constraint.as_ptr(),
      params[0],
      builtin_types.string_type,
    );
  }

  let optional_number = unsafe {
    let builtin_types = &*solver.builtin_types;
    arena.add_type(UnionType {
      options: vec![builtin_types.nil_type, builtin_types.number_type],
    })
  };

  let init_index = if call_site.self_ { 1 } else { 2 };
  if params.len() == 3 && call_site.args.size > init_index {
    unsafe {
      (*context.solver.as_ptr()).constraint_solver_unify(
        context.constraint.as_ptr(),
        params[2],
        optional_number,
      );
    }
  }

  let return_list = arena.add_type_pack_t(TypePack {
    head: return_types,
    tail: None,
  });
  let result_mut = as_mutable_type_pack(context.result);
  unsafe {
    (*result_mut).ty = TypePackVariant::Bound(return_list);
  }

  true
}
