use ulua_common::{LUAU_ASSERT, fflag};

use crate::{
  functions::optional_node::slot_opt,
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_stat_class::AstStatClass, parser::Parser,
  },
  rtti::ast_node_try_as,
};

impl Parser {
  /// 对应 cpp Parser.cpp:973 `getMatchingClass`：全局名命中已声明类时返回该类
  /// 声明，供 l-value 判定与报错取定义行。返回 `Option` 后 null 哨兵消失，
  /// 消费点判空守卫一并折叠为 `is_some`/`if let`。
  pub(crate) fn get_matching_class(&self, expr: *mut AstExpr) -> Option<&'static AstStatClass> {
    LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());
    // `slot_opt` 折叠 null/存活两步判空（cpp 的 expr 判空由调用方保证，这里再兜
    // 一层不误判），命中 AstExprGlobal 才继续查类表；类表值是 parser 登记进 arena
    // 的存活 AstStatClass 节点或 null（未登记槽位），同样折叠进 Option。
    match slot_opt(expr).and_then(|expr| ast_node_try_as::<AstExprGlobal>(&expr.base)) {
      Some(global) => self
        .classes_within_module
        .find(&global.name)
        .copied()
        .and_then(|class| slot_opt(class)),
      None => None,
    }
  }
}
