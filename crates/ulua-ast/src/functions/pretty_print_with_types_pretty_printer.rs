use alloc::string::String;

use crate::{
  records::{
    ast_stat_block::AstStatBlock, position::Position, printer::Printer, string_writer::StringWriter,
  },
  type_aliases::cst_node_map::CstNodeMap,
};

pub fn pretty_print_with_types_ast_stat_block_cst_node_map(
  block: &mut AstStatBlock,
  cst_node_map: CstNodeMap,
) -> String {
  let mut writer = StringWriter {
    ss: String::new(),
    pos: Position::default(),
    last_char: ' ',
  };
  let mut printer = Printer::new(&mut writer, cst_node_map);
  printer.write_types = true;
  printer.visualize_block_ast_stat_block(block);
  writer.str().clone()
}
