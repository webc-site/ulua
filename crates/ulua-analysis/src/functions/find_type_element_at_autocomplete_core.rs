use ulua_ast::{
  records::{
    ast_node::AstNode, ast_type_list::AstTypeList, ast_type_pack_variadic::AstTypePackVariadic,
    position::Position,
  },
  rtti::ast_node_as,
};

use crate::{
  functions::{
    find_type_element_at_autocomplete_core_alt_c::find_type_element_at_ast_type_type_id_position,
    flatten_type_pack::flatten_type_pack_id, follow_type_pack::follow_type_pack_id,
    get_type_pack::get_type_pack_id,
  },
  records::variadic_type_pack::VariadicTypePack,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
pub fn find_type_element_at_ast_type_list_type_pack_id_position(
  ast_type_list: &AstTypeList,
  tp: TypePackId,
  position: Position,
) -> Option<TypeId> {
  let types = ast_type_list.types.as_slice();

  for (i, &type_) in types.iter().enumerate() {
    let location = unsafe { (*type_).base.location };
    if location.contains_closed(position) {
      let (head, _) = flatten_type_pack_id(tp);

      if i < head.len() {
        return unsafe { find_type_element_at_ast_type_type_id_position(type_, head[i], position) };
      }
    }
  }

  if !ast_type_list.tail_type.is_null() {
    let arg_tp = unsafe { &*ast_type_list.tail_type };

    let variadic =
      unsafe { ast_node_as::<AstTypePackVariadic>(&arg_tp.base as *const AstNode as *mut AstNode) };
    if !variadic.is_null() {
      let location = unsafe { (*variadic).base.base.location };
      if location.contains_closed(position) {
        let (_, tail) = flatten_type_pack_id(tp);

        if let Some(tail_id) = tail {
          let follow_tp = unsafe { follow_type_pack_id(tail_id) };
          if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(follow_tp) {
            let variadic_type = unsafe { (*variadic).variadic_type };
            return unsafe {
              find_type_element_at_ast_type_type_id_position(variadic_type, vtp.ty, position)
            };
          }
        }
      }
    }
  }

  None
}
