use ulua_ast::{
  records::{
    ast_node::AstNode, ast_type::AstType, ast_type_function::AstTypeFunction, position::Position,
  },
  rtti::{ast_node_as, ast_rtti_index},
};

use crate::{
  functions::{
    find_type_element_at_autocomplete_core::find_type_element_at_ast_type_list_type_pack_id_position,
    find_type_element_at_autocomplete_core_alt_b::find_type_element_at_ast_type_pack_type_pack_id_position,
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
  },
  records::function_type::FunctionType,
  type_aliases::type_id::TypeId,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn find_type_element_at_ast_type_type_id_position(
  ast_type: *mut AstType,
  ty: TypeId,
  position: Position,
) -> Option<TypeId> {
  let ty = follow_type_id(ty);

  unsafe {
    if (*ast_type).base.class_index == ast_rtti_index("AstTypeReference") {
      return Some(ty);
    }

    if (*ast_type).base.class_index == ast_rtti_index("AstTypeError") {
      return Some(ty);
    }

    let type_function = ast_node_as::<AstTypeFunction>(ast_type as *mut AstNode);
    if !type_function.is_null() {
      let ftv = get_type_id::<FunctionType>(ty)?;

      let arg_types = &(*type_function).arg_types;
      let arg_types_tp = ftv.arg_types;

      if let Some(element) =
        find_type_element_at_ast_type_list_type_pack_id_position(arg_types, arg_types_tp, position)
      {
        return Some(element);
      }

      let return_types = (*type_function).return_types;
      let ret_types_tp = ftv.ret_types;

      if let Some(element) = find_type_element_at_ast_type_pack_type_pack_id_position(
        return_types,
        ret_types_tp,
        position,
      ) {
        return Some(element);
      }
    }
  }

  None
}
