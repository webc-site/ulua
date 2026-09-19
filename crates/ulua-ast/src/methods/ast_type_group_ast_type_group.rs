use crate::{
  records::{
    ast_node::AstNode, ast_type::AstType, ast_type_group::AstTypeGroup, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstTypeGroup {
  pub fn new(location: Location, type_: *mut AstType) -> Self {
    Self {
      base: AstType {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      type_,
    }
  }
}
