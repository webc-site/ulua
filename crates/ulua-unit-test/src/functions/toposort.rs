use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_analysis::functions::toposort::toposort as analysis_toposort;
use ulua_ast::records::{ast_stat::AstStat, ast_stat_block::AstStatBlock, node_handle::Node};

/// 测试门面:以句柄形态返回置换结果,内部仍走 analysis 的裸指针排序入口。
pub fn toposort(block: &mut AstStatBlock) -> Vec<Node<AstStat>> {
  let mut result: Vec<*mut AstStat> = block.body.iter_nodes().map(|n| n.as_ptr()).collect();
  analysis_toposort(&mut result);
  result
    .into_iter()
    // toposort 只置换既有非空槽位,判空拦截即可安全建槽。
    .map(|p| Node::from_non_null(NonNull::new(p).expect("toposort 输出槽位恒非空")))
    .collect()
}
