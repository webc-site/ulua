use core::ptr::{null, null_mut};

use ulua_ast::{
  records::{
    ast_stat_type_alias::AstStatTypeAlias, ast_type::AstType, ast_type_reference::AstTypeReference,
  },
  rtti::ast_node_as,
};

use crate::records::type_map_visitor::TypeMapVisitor;

impl<'a> TypeMapVisitor<'a> {
  pub fn resolve_aliases_deprecated(&mut self, ty: *const AstType) -> *const AstType {
    type_map_visitor_resolve_aliases_deprecated(self, ty)
  }
}

pub fn type_map_visitor_resolve_aliases_deprecated<'a>(
  this: &mut TypeMapVisitor<'a>,
  ty: *const AstType,
) -> *const AstType {
  if ty.is_null() {
    return null();
  }

  unsafe {
    let type_ref = ast_node_as::<AstTypeReference>(&mut *(ty as *mut AstType as *mut _));
    if !type_ref.is_null() {
      let ref_node = &*type_ref;

      if ref_node.prefix.is_some() {
        return ty;
      }

      let alias = if let Some(alias_ptr) = this.type_aliases.find(&ref_node.name).copied() {
        alias_ptr
      } else {
        null_mut()
      };

      if !alias.is_null() {
        let alias_ref: &AstStatTypeAlias = &*alias;
        return alias_ref.type_ptr as *const AstType;
      }
    }
  }

  ty
}
