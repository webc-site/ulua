use crate::{
  records::{
    ast_array::AstArray, ast_node::AstNode, ast_type::AstType,
    ast_type_intersection::AstTypeIntersection, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstTypeIntersection {
  pub fn new(location: Location, types: AstArray<*mut AstType>) -> Self {
    Self {
      base: AstType {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      types,
    }
  }
}
