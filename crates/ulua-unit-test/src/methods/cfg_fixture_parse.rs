use ulua_ast::records::{
  ast_stat_block::AstStatBlock, parse_errors::ParseErrors, parse_options::ParseOptions,
  parser::Parser,
};

use crate::records::cfg_fixture::CfgFixture;

impl CfgFixture {
  pub fn parse(&mut self, code: &str) -> *mut AstStatBlock {
    self.names.rebind_allocator(&mut self.allocator as *mut _);

    let result = Parser::parse(
      code,
      code.len(),
      &mut self.names,
      &mut self.allocator,
      ParseOptions::default(),
    );

    if !result.errors.is_empty() {
      panic!("{}", ParseErrors::new(result.errors));
    }

    result.root
  }
}
