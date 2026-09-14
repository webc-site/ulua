use crate::{
  records::{
    ast_array::AstArray,
    ast_attr::{AstAttr, AstAttrType},
    ast_expr::AstExpr,
    ast_name::AstName,
    ast_node::AstNode,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstAttr {
  pub fn ast_attr_location_type_item_ast_array_ast_expr_ast_name(
    location: Location,
    r#type: AstAttrType,
    args: AstArray<*mut AstExpr>,
    name: AstName,
  ) -> Self {
    Self {
      base: AstNode {
        class_index: <Self as AstNodeClass>::CLASS_INDEX,
        location,
      },
      r#type,
      args,
      name,
    }
  }
}
