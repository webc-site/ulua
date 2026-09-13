use alloc::string::String;
use core::ptr::null_mut;

use crate::{
  functions::pretty_print_with_types_pretty_printer::pretty_print_with_types_ast_stat_block_cst_node_map,
  records::ast_stat_block::AstStatBlock, type_aliases::cst_node_map::CstNodeMap,
};

pub fn pretty_print_with_types_ast_stat_block(block: &mut AstStatBlock) -> String {
  let cst_node_map = CstNodeMap::new(null_mut());
  pretty_print_with_types_ast_stat_block_cst_node_map(block, cst_node_map)
}
