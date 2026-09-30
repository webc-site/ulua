use crate::records::{
  ast_array::AstArray, ast_type::AstType, ast_type_union::AstTypeUnion, location::Location,
};

impl_ast_node_new!(AstTypeUnion, AstType, location: Location, types: AstArray<*mut AstType>);
