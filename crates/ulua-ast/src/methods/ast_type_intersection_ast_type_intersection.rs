use crate::records::{
  ast_array::AstArray, ast_type::AstType, ast_type_intersection::AstTypeIntersection,
  location::Location,
};

impl_ast_node_new!(AstTypeIntersection, AstType, location: Location, types: AstArray<*mut AstType>);
