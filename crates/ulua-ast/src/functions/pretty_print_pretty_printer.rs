use alloc::string::String;

use crate::{
  enums::quote_style_cst::QuoteStyle,
  records::{
    ast_stat_block::AstStatBlock, position::Position, printer::Printer,
    string_writer::StringWriter, writer::Writer,
  },
  type_aliases::cst_node_map::CstNodeMap,
};

pub fn pretty_print_ast_stat_block_cst_node_map(
  block: &mut AstStatBlock,
  cst_node_map: CstNodeMap,
) -> String {
  let mut writer = StringWriter {
    ss: String::new(),
    pos: Position::new(0, 0),
    last_char: '\0',
  };

  {
    // Printer::new takes &mut dyn Writer. StringWriter implements Writer.
    let mut printer = Printer::new(&mut writer, cst_node_map);
    printer.visualize_block_ast_stat_block(block);
  }

  // writer.str() returns &String, we need to return an owned String.
  writer.str().clone()
}

impl Writer for StringWriter {
  fn advance(&mut self, new_pos: &Position) {
    self.advance(new_pos);
  }

  fn maybe_space(&mut self, new_pos: &Position, reserve: i32) {
    self.maybe_space(new_pos, reserve);
  }

  fn newline(&mut self) {
    self.newline();
  }

  fn space(&mut self) {
    self.space();
  }

  fn write_multiline(&mut self, s: &str) {
    self.write_multiline(s);
  }

  fn write(&mut self, s: &str) {
    self.write(s);
  }

  fn identifier(&mut self, s: &str) {
    self.identifier(s);
  }

  fn keyword(&mut self, s: &str) {
    self.keyword(s);
  }

  fn symbol(&mut self, s: &str) {
    self.symbol(s);
  }

  fn literal(&mut self, s: &str) {
    self.literal(s);
  }

  fn string(&mut self, s: &str) {
    self.string(s);
  }

  fn source_string(&mut self, s: &str, quote_style: QuoteStyle, block_depth: u32) {
    self.source_string(s, quote_style, block_depth);
  }
}
