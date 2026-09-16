extern crate alloc;

use alloc::string::String;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::dump_instruction::dump_instruction, records::block::Block,
  type_aliases::definition::Definition,
};

pub fn dump_block(block: &Block, use_defs: &DenseHashMap<*mut AstExpr, *mut Definition>) -> String {
  let mut result = String::new();
  for inst in block.get_instructions() {
    result.push_str("  ");
    result.push_str(&unsafe { dump_instruction(*inst, use_defs) });
    result.push('\n');
  }
  result
}
