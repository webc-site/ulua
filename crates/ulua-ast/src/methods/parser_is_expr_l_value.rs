use ulua_common::fflag;

use crate::{
  enums::ast_expr_ref::AstExprRef,
  functions::optional_node::slot_opt,
  records::{ast_expr::AstExpr, parser::Parser},
};

impl Parser {
  /// 对应 cpp Parser.cpp:984 `isExprLValue`：local（非 const）、global（未命中
  /// 已声明类）、IndexName/IndexExpr 四类可作赋值目标。
  pub(crate) fn is_expr_l_value(&self, expr: *mut AstExpr) -> bool {
    // expr 契约为 arena 存活节点或 null；`slot_opt` 折叠 None 早退（等价旧判空），
    // Some 侧取 repr(C) 基类前缀只读分派。
    let Some(expr_ref) = slot_opt(expr) else {
      return false;
    };

    match expr_ref.as_expr_ref() {
      AstExprRef::Local(local) => !local.local.get().is_const,
      AstExprRef::Global(_) => {
        !(fflag::DebugLuauUserDefinedClasses.get() && self.get_matching_class(expr).is_some())
      }
      AstExprRef::IndexName(_) | AstExprRef::IndexExpr(_) => true,
      _ => false,
    }
  }
}
