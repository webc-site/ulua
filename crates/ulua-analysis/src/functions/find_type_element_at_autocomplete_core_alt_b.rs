use ulua_ast::{
  records::{
    ast_node::AstNode, ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_pack_variadic::AstTypePackVariadic, position::Position,
  },
  rtti::ast_node_is,
};

use crate::{
  functions::{
    find_type_element_at_autocomplete_core::find_type_element_at_ast_type_list_type_pack_id_position,
    find_type_element_at_autocomplete_core_alt_c::find_type_element_at_ast_type_type_id_position,
    flatten_type_pack::flatten_type_pack_id, follow_type_pack::follow_type_pack_id,
    get_type_pack::get_type_pack_id,
  },
  records::variadic_type_pack::VariadicTypePack,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn find_type_element_at_ast_type_pack_type_pack_id_position(
  ast_type_pack: *mut AstTypePack,
  tp: TypePackId,
  position: Position,
) -> Option<TypeId> {
  unsafe {
    if !ast_type_pack.is_null() {
      let node = ast_type_pack as *mut AstNode;
      if ast_node_is::<AstTypePackExplicit>(&*node) {
        let explicit = ast_type_pack as *mut AstTypePackExplicit;
        let type_list = (*explicit).type_list;
        return find_type_element_at_ast_type_list_type_pack_id_position(&type_list, tp, position);
      } else if ast_node_is::<AstTypePackVariadic>(&*node) {
        let variadic = ast_type_pack as *mut AstTypePackVariadic;
        let loc = (*ast_type_pack).base.location;
        if loc.contains_closed(position) {
          let (_, tail) = flatten_type_pack_id(tp);

          if let Some(tail_id) = tail {
            let follow_tp = follow_type_pack_id(tail_id);
            if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(follow_tp) {
              let variadic_type = (*variadic).variadic_type;
              return find_type_element_at_ast_type_type_id_position(
                variadic_type,
                vtp.ty,
                position,
              );
            }
          }
        }
      }
    }
  }
  None
}
