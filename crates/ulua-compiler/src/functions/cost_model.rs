use core::ptr::from_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_local::AstLocal, ast_node::AstNode,
  },
  visit::ast_node_visit,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{constant::Constant, cost_visitor::CostVisitor, node::Node};

/// 单个函数最多参与成本折扣的变量数（C++ `varCount < 7`）
pub(crate) const K_MAX_COST_VARS: usize = 7;
/// 变量折扣位宽
pub(crate) const K_VAR_DISCOUNT_BITS: usize = 8;

/// 对应 cpp `modelCost`（cpp/Compiler/src/CostModel.cpp:425）：以 `CostVisitor`
/// 遍历 `root`（函数体/循环体语句块向上转 `AstNode`）估算内联成本；`vars` 指针切片
/// 为参与折扣的形参/循环变量（至多 7 个，cpp `varCount < 7`），仅作 map 地址键。
pub fn model_cost(
  root: &mut AstNode,
  vars: &[Node<AstLocal>],
  builtins: &DenseHashMap<Node<AstExprCall>, i32>,
  constants: &DenseHashMap<Node<AstExpr>, Constant>,
) -> u64 {
  let mut visitor = CostVisitor::new(builtins, constants);

  // cpp `modelCost` 双指针游标 (vars, varCount) 收口为切片：元素均指向
  // 存活 AstLocal（调用方传函数 args 等 arena 数组或栈上单元素切片），
  // min(7) 只缩短读取半径；切片仅读取指针值作 map 键。
  for (i, &var_ptr) in vars.iter().take(K_MAX_COST_VARS).enumerate() {
    *visitor.vars.get_or_insert(var_ptr) =
      0xffu64 << ((i * K_VAR_DISCOUNT_BITS + K_VAR_DISCOUNT_BITS) as u32);
  }

  // Safety: `from_mut(root)` 交出调用方独占借用的 arena 节点地址（parser 分配、
  // 编译结束前稳定）；`ast_node_visit` 沿子指针只读遍历，`CostVisitor` 只写自身
  // result/vars/loops 表，不写 AST，独占成立。
  unsafe { ast_node_visit(from_mut(root), &mut visitor) };

  visitor.result.model
}
