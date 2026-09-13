use ulua_ast::records::{allocator::Allocator, ast_name_table::AstNameTable};

use crate::records::test_file_resolver::TestFileResolver;

#[derive(Debug)]
#[repr(C)]
pub struct RequireTracerFixture {
  pub allocator: Box<Allocator>,
  pub names: Box<AstNameTable>,
  pub file_resolver: TestFileResolver,
}
