use alloc::string::String;

use crate::{
  functions::pretty_print_pretty_printer::pretty_print_to_string,
  records::ast_stat_block::AstStatBlock, type_aliases::cst_node_map::CstNodeMap,
};

pub fn pretty_print_with_types_ast_stat_block_cst_node_map(
  block: &mut AstStatBlock,
  cst_node_map: &CstNodeMap,
) -> String {
  pretty_print_to_string(block, cst_node_map, true)
}
