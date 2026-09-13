use alloc::sync::Arc;

use ulua_ast::{
  enums::mode::Mode,
  records::{parse_options::ParseOptions, parse_result::ParseResult, parser::Parser},
};

use crate::records::source_module::SourceModule;
pub fn parse_source_for_module(
  source: &str,
  source_module: &mut SourceModule,
  capture_comments: bool,
) -> ParseResult {
  let options = ParseOptions {
    allow_declaration_syntax: true,
    capture_comments,
    ..Default::default()
  };

  let parse_result = Parser::parse(
    source,
    source.len(),
    Arc::get_mut(&mut source_module.names)
      .expect("SourceModule names must be uniquely owned while parsing"),
    Arc::get_mut(&mut source_module.allocator)
      .expect("SourceModule allocator must be uniquely owned while parsing"),
    options.clone(),
  );

  source_module.root = parse_result.root;
  source_module.mode = Some(Mode::Definition);

  if options.capture_comments {
    source_module.hotcomments = parse_result.hotcomments.clone();
    source_module.comment_locations = parse_result.comment_locations.clone();
  }

  parse_result
}
