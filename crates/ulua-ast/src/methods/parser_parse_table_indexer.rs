use crate::{
  enums::ast_table_access::AstTableAccess,
  records::{
    ast_table_indexer::AstTableIndexer, lexeme::Lexeme, location::Location, parser::Parser,
    position::Position, table_indexer_result::TableIndexerResult,
  },
};

impl Parser {
  pub fn parse_table_indexer(
    &mut self,
    access: AstTableAccess,
    access_location: Option<Location>,
    begin: Lexeme,
  ) -> TableIndexerResult {
    let index = self.parse_type_bool(false);

    let indexer_close_found = self.expect_and_consume_char(']', "table field");
    let indexer_close_position = if indexer_close_found {
      self.lexer.previous_location().begin
    } else {
      Position::missing()
    };

    let colon_found = self.expect_and_consume_char(':', "table field");
    let colon_position = if colon_found {
      self.lexer.previous_location().begin
    } else {
      Position::missing()
    };

    let result = self.parse_type_bool(false);

    let node = unsafe {
      (*self.allocator).alloc(AstTableIndexer {
        index_type: index,
        result_type: result,
        location: Location::new(begin.location.begin, (*result).base.location.end),
        access,
        access_location,
      })
    };

    TableIndexerResult {
      node,
      indexer_open_position: begin.location.begin,
      indexer_close_position,
      colon_position,
    }
  }
}
