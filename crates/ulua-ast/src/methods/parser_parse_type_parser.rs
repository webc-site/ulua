use ulua_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

use crate::records::{
  allocator::Allocator, ast_name_table::AstNameTable, ast_type::AstType,
  parse_node_result::ParseNodeResult, parse_options::ParseOptions, parser::Parser,
};

impl Parser {
  pub fn parse_type_c_char_usize_ast_name_table_allocator_parse_options<B>(
    buffer: &B,
    names: &mut AstNameTable,
    allocator: &mut Allocator,
    options: ParseOptions,
  ) -> ParseNodeResult<AstType>
  where
    B: AsRef<[u8]> + ?Sized,
  {
    LUAU_TIMETRACE_SCOPE!("Parser::parseType", "Parser");

    Parser::run_parse::<AstType, _>(buffer.as_ref(), names, allocator, options, |parser| {
      parser.parse_type(false)
    })
  }
}
