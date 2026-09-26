use crate::{
  records::{
    ast_attr::{AstAttr, AstAttrType},
    ast_expr::AstExpr,
    ast_name::AstName,
    ast_node::AstNode,
    location::Location,
    node_handle::Nodes,
  },
  rtti::AstNodeClass,
};

impl AstAttr {
  pub fn ast_attr_location_type_item_ast_array_ast_expr_ast_name(
    location: Location,
    r#type: AstAttrType,
    args: Nodes<AstExpr>,
    name: AstName,
  ) -> Self {
    Self {
      base: AstNode::new(<Self as AstNodeClass>::CLASS_INDEX, location),
      r#type,
      args,
      name,
    }
  }
}
