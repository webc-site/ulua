use ulua_common::{LUAU_ASSERT, fflag};

use crate::{
  functions::optional_node::slot_opt,
  records::{
    ast_expr::AstExpr,
    ast_expr_global::AstExprGlobal,
    ast_stat_class::AstStatClass,
    node_handle::{Node, OptNode},
    parser::Parser,
  },
  rtti::ast_node_try_as,
};

impl Parser {
  /// 对应 cpp Parser.cpp:973 `getMatchingClass`：全局名命中已声明类时返回该类
  /// 声明，供 l-value 判定与报错取定义行。返回 `Option` 后 null 哨兵消失，
  /// 消费点判空守卫一并折叠为 `is_some`/`if let`。
  ///
  /// 值面交回**自有**的 [`Node`]（`Copy`，不携带 `&self` 借用）：调用方随后仍可
  /// 以 `&mut self` 报错，无需再把 arena 引用铸成假的 `'static`。
  pub(crate) fn get_matching_class(&self, expr: *mut AstExpr) -> Option<Node<AstStatClass>> {
    LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());
    // `slot_opt` 折叠 null/存活两步判空（cpp 的 expr 判空由调用方保证，这里再兜
    // 一层不误判），命中 AstExprGlobal 才继续查类表；类表值列是 cpp `operator[]`
    // 形态的可空槽（登记时立即接线，null 槽只在同名类未登记的中间态出现），
    // `to_option` 把「槽位为空」与「未命中」一并折成 None，与旧 `slot_opt` 等价。
    match slot_opt(expr).and_then(|expr| ast_node_try_as::<AstExprGlobal>(&expr.base)) {
      Some(global) => self
        .classes_within_module
        .find(&global.name)
        .and_then(OptNode::to_option),
      None => None,
    }
  }
}
