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
    function_type::FunctionType, magic_function_call_context::MagicFunctionCallContext,
    magic_gmatch::MagicGmatch, type_pack::TypePack,
  },
  type_aliases::type_pack_variant::TypePackVariant,
};
impl MagicGmatch {
  pub fn infer(&self, context: &MagicFunctionCallContext) -> bool {
    let (params, _tail) = flatten_type_pack_id(context.arguments);

    if params.len() != 2 {
      return false;
    }

    let solver = unsafe { context.solver.as_ref() };
    let arena = unsafe { &mut *solver.arena };
    let call_site = unsafe { context.call_site.as_ref() };

    let index = if call_site.self_ { 0 } else { 1 };
    let pattern = if call_site.args.size > index {
      let expr = unsafe { *call_site.args.data.add(index) };
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

    let empty_pack = arena.add_type_pack_t(TypePack {
      head: Vec::new(),
      tail: None,
    });
    let return_list = arena.add_type_pack_t(TypePack {
      head: return_types,
      tail: None,
    });
    let iterator_type = arena.add_type(FunctionType::function_type_new(
      empty_pack,
      return_list,
      None,
      false,
    ));
    let res_type_pack = arena.add_type_pack_t(TypePack {
      head: vec![iterator_type],
      tail: None,
    });

    let result_mut = as_mutable_type_pack(context.result);
    unsafe {
      (*result_mut).ty = TypePackVariant::Bound(res_type_pack);
    }

    true
  }
}
