use alloc::vec::Vec;

use ulua_analysis::functions::toposort::toposort as analysis_toposort;
use ulua_ast::records::{ast_stat::AstStat, ast_stat_block::AstStatBlock, node_handle::Node};

/// 测试门面:以 arena 句柄形态返回置换结果,与 analysis 的句柄排序入口同型直传,
/// 不再经裸指针往返。
pub fn toposort(block: &mut AstStatBlock) -> Vec<Node<AstStat>> {
  let stats: Vec<Node<AstStat>> = block.body.iter_nodes().copied().collect();
  analysis_toposort(&stats)
}
