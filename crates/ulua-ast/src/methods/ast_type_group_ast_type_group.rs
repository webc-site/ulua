use crate::records::{ast_type::AstType, ast_type_group::AstTypeGroup, location::Location};

impl_ast_node_new!(AstTypeGroup, AstType, location: Location, type_: *mut AstType);
