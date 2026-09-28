use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_expr_table::AstExprTable,
  },
  rtti::ast_node_try_as,
};

use crate::{functions::ast_slot_ref::ast_slot_ref, records::node::Node};

/// C++ `getTableHint`：`expr` 为表字面量，或 `setmetatable(表字面量, ...)` 时
/// 返回内层表指针（供表形状预测用），否则 `None`（原 null 哨兵已 Option 化）。
pub(crate) fn get_table_hint(expr: Node<AstExpr>) -> Option<Node<AstExprTable>> {
  // expr 为表形状预测传入的 arena 存活 AstExpr 句柄（visit_stat_local 的
  // values 元素，契约见 `Node::borrow`）；判型走安全引用门面。
  let expr_ref = expr.borrow();
  if let Some(table) = ast_node_try_as::<AstExprTable>(expr_ref) {
    return Some(table.into());
  }

  if let Some(call) = ast_node_try_as::<AstExprCall>(expr_ref)
    && !call.self_
    && call.args.size == 2
    // call.func 为 parser 保证非空存活的被调表达式指针，`ast_slot_ref` 收口
    // 判空与只读解引用。
    && let Some(func) = ast_slot_ref(call.func).and_then(|f| ast_node_try_as::<AstExprGlobal>(f))
    && func.name == "setmetatable"
  {
    // 上一行已验证 args.size == 2：首槽经 as_slice 界内取回；arg0 为 args
    // 数组内 parser 写入的非空存活表达式指针。
    let arg0 = call.args.as_slice()[0];
    if let Some(table_arg) = ast_slot_ref(arg0).and_then(|a| ast_node_try_as::<AstExprTable>(a)) {
      return Some(table_arg.into());
    }
  }

  None
}
