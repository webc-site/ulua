use ulua_ast::{
  records::{ast_expr_table::AstExprTable, ast_local::AstLocal, ast_node::AstNode},
  visit::dispatch_node,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::table_constant_kind::TableConstantKind,
  functions::unwrap_expr_of_type::unwrap_expr_of_type,
  records::{node::Node, table_mutation_tracker::TableMutationTracker, variable::Variable},
};

/// 对应 cpp `buildTableConstantMap`（cpp/Compiler/src/ConstantFolding.cpp:1245）：
/// 用 `TableMutationTracker` 遍历 `root` 子树标记逃逸的 local，随后把未逃逸且初值为
/// 表字面量的 `variables` 收集为 `ConstantTable` 映射返回。`variables` 的
/// `*mut AstLocal` 键均来自 `track_values` 对同一棵树的登记。
pub(crate) fn build_table_constant_map(
  variables: &DenseHashMap<Node<AstLocal>, Variable>,
  root: &mut AstNode,
) -> DenseHashMap<Node<AstLocal>, TableConstantKind> {
  let mut result = DenseHashMap::default();
  // cpp/Compiler/src/ConstantFolding.cpp:1245 `buildTableConstantMap` 只有 TableMutationTracker
  // 单一路径（旧 `LuauCompileNewTableMutationTracker` 旗标与 deprecated tracker 上游已删除）。
  let mut tracker = TableMutationTracker::new();
  // `dispatch_node` 只沿 parser 接线子指针短读遍历调用方独占的 `root`，与 `tracker`
  // 对只读 `variables` 的独占借用无别名交集。后续 `var.init` 读取经 Option 命中才交
  // unwrap_expr_of_type 使用。
  dispatch_node(root, &mut tracker);

  for (local, var) in variables.iter() {
    if var.written {
      continue;
    }

    // cpp `var->init == nullptr` 判空 → Option（无初值即非表常量候选）
    let Some(init) = var.init else {
      continue;
    };
    if unwrap_expr_of_type::<AstExprTable>(init).is_none() {
      continue;
    }

    if !tracker.escaped.contains(local) {
      *result.get_or_insert(*local) = TableConstantKind::ConstantTable;
    }
  }

  result
}
