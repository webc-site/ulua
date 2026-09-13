use alloc::vec::Vec;
use core::slice::from_raw_parts_mut;

use ulua_analysis::functions::toposort::toposort as analysis_toposort;
use ulua_ast::records::{ast_stat::AstStat, ast_stat_block::AstStatBlock};

pub fn toposort(block: &mut AstStatBlock) -> Vec<*mut AstStat> {
  // 块体裸指针数组转切片视图，收集语句指针
  let mut result: Vec<*mut AstStat> =
    unsafe { from_raw_parts_mut(block.body.data, block.body.size) }.to_vec();

  analysis_toposort(&mut result);

  result
}
