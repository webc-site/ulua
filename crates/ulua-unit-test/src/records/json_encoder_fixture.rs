use ulua_ast::records::{allocator::Allocator, ast_name_table::AstNameTable};

#[derive(Debug)]
#[repr(C)]
pub struct JsonEncoderFixture {
  pub allocator: Allocator,
  pub names: AstNameTable,
}

impl JsonEncoderFixture {
  pub fn new() -> Self {
    let mut allocator = Allocator::new();
    let names = AstNameTable::new(&mut allocator);
    Self { allocator, names }
  }
}

impl Default for JsonEncoderFixture {
  fn default() -> Self {
    Self::new()
  }
}
