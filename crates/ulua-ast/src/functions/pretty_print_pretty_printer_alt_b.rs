use alloc::string::String;
use core::ptr::null_mut;

use crate::{
  functions::pretty_print_pretty_printer::pretty_print_ast_stat_block_cst_node_map,
  records::ast_stat_block::AstStatBlock, type_aliases::cst_node_map::CstNodeMap,
};

pub fn pretty_print_ast_stat_block(block: &mut AstStatBlock) -> String {
  pretty_print_ast_stat_block_cst_node_map(block, CstNodeMap::new(null_mut()))
}
