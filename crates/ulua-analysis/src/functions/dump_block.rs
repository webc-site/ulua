extern crate alloc;

use alloc::string::String;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::dump_instruction::dump_instruction, records::block::Block,
  type_aliases::def_id_control_flow_graph::DefId,
};

pub fn dump_block(block: &Block, use_defs: &DenseHashMap<*mut AstExpr, DefId>) -> String {
  let mut result = String::new();
  for inst in block.get_instructions() {
    result.push_str("  ");
    // instr 为本构建期 register_instruction 发放的存活句柄（见 `instr_registry`
    // 模块契约），dump_instruction 经注册表解析后全程只读指令与 use_defs 映射，
    // 单线程转储无别名冲突，句柄按值拷贝即可。
    result.push_str(&dump_instruction(*inst, use_defs));
    result.push('\n');
  }
  result
}
