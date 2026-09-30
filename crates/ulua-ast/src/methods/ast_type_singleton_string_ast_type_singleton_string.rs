use crate::records::{
  ast_array::AstArray, ast_type::AstType, ast_type_singleton_string::AstTypeSingletonString,
  location::Location,
};

impl_ast_node_new!(AstTypeSingletonString, AstType, location: Location, value: AstArray<u8>);
