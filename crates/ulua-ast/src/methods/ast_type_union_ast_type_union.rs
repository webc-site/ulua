use crate::{
  records::{
    ast_array::AstArray, ast_node::AstNode, ast_type::AstType, ast_type_union::AstTypeUnion,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstTypeUnion {
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

pub fn ast_type_union_ast_type_union(
  location: Location,
  types: AstArray<*mut AstType>,
) -> AstTypeUnion {
  AstTypeUnion::new(location, types)
}
