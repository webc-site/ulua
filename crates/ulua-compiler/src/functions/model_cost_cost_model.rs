use alloc::vec::Vec;

use ulua_ast::records::{ast_local::AstLocal, ast_node::AstNode};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{functions::cost_model::model_cost, records::node::Node};

/// 对应 cpp `modelCost(root, vars, varCount)` 便捷重载
/// （cpp/Compiler/src/CostModel.cpp:442）：crate 内外统一以引用/切片表达
/// (ptr, count) 双指针协议，直接转发 [`model_cost`]。
pub fn model_cost_ast_node_ast_local_usize(root: &mut AstNode, vars: &[*mut AstLocal]) -> u64 {
  let builtins = DenseHashMap::default();
  let constants = DenseHashMap::default();
  // C ABI 测试入口保留裸指针形参，边界处一次性升为地址句柄
  let vars: Vec<Node<AstLocal>> = vars.iter().copied().map(Node::new).collect();

  model_cost(root, &vars, &builtins, &constants)
}
