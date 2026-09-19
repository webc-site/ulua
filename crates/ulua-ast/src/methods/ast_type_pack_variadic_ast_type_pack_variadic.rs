use crate::{
  records::{
    ast_node::AstNode, ast_type::AstType, ast_type_pack::AstTypePack,
    ast_type_pack_variadic::AstTypePackVariadic, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstTypePackVariadic {
  pub fn new(location: Location, variadic_type: *mut AstType) -> Self {
    Self {
      base: AstTypePack {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      variadic_type,
    }
  }
}
