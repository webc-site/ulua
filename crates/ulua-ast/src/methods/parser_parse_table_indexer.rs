use crate::{
  enums::ast_table_access::AstTableAccess,
  records::{
    ast_table_indexer::AstTableIndexer, lexeme::Lexeme, location::Location, parser::Parser,
    table_indexer_result::TableIndexerResult,
  },
};

impl Parser {
  pub fn parse_table_indexer(
    &mut self,
    access: AstTableAccess,
    access_location: Option<Location>,
    begin: Lexeme,
  ) -> TableIndexerResult {
    let index = self.parse_type(false);

    let indexer_close_position = self.expect_and_consume_char_position(']', "table field");
    let colon_position = self.expect_and_consume_char_position(':', "table field");

    let result = self.parse_type(false);

    let node = self.alloc(AstTableIndexer {
      index_type: index,
      result_type: result,
      location: Location::new(begin.location.begin, unsafe {
        // Safety: result 为 parse_type 刚在 arena 分配的存活类型节点（alloc_type 恒非空，失败中止），(*result).base.location.end 为基类前缀字段只读拷贝。
        (*result).base.location.end
      }),
      access,
      access_location,
    });

    TableIndexerResult {
      node,
      indexer_open_position: begin.location.begin,
      indexer_close_position,
      colon_position,
    }
  }
}
