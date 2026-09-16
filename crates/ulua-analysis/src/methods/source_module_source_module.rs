use alloc::sync::Arc;
use core::ptr::null_mut;

use ulua_ast::records::{allocator::Allocator, ast_name_table::AstNameTable};

use crate::records::{source_code::SourceCode, source_module::SourceModule};
impl SourceModule {
  pub fn new() -> Self {
    let mut allocator = Arc::new(Allocator::new());
    let names = AstNameTable::new(
      Arc::get_mut(&mut allocator).expect("fresh SourceModule allocator must be unique"),
    );
    Self {
      name: String::new(),
      human_readable_name: String::new(),
      r#type: SourceCode::NONE,
      environment_name: None,
      cyclic: false,
      allocator,
      names: Arc::new(names),
      parse_errors: Vec::new(),
      root: null_mut(),
      mode: None,
      hotcomments: Vec::new(),
      comment_locations: Vec::new(),
    }
  }
}

impl Default for SourceModule {
  fn default() -> Self {
    Self::new()
  }
}
