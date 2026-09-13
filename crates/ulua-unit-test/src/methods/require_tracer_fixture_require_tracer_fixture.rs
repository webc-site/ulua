use ulua_ast::records::{allocator::Allocator, ast_name_table::AstNameTable};

use crate::records::{
  require_tracer_fixture::RequireTracerFixture, test_file_resolver::TestFileResolver,
};

pub fn require_tracer_fixture_require_tracer_fixture() -> RequireTracerFixture {
  let mut allocator = Box::new(Allocator::new());
  let names = Box::new(AstNameTable::new(&mut allocator));

  RequireTracerFixture {
    allocator,
    names,
    file_resolver: TestFileResolver::default(),
  }
}
