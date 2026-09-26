use ulua_common::fflag;

use crate::{
  functions::optional_node::slot_opt,
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, parser::Parser,
  },
  rtti::{AstNodeClass, ast_node_as_unchecked},
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

    let node = &expr_ref.base;
    match node.class_index {
      AstExprLocal::CLASS_INDEX => {
        let local: &AstExprLocal = unsafe { ast_node_as_unchecked(node) };
        // local 槽已句柄化恒非空：.get() 安全借用，判空折叠随类型消失。
        !local.local.get().is_const
      }
      AstExprGlobal::CLASS_INDEX => {
        !(fflag::DebugLuauUserDefinedClasses.get() && self.get_matching_class(expr).is_some())
      }
      AstExprIndexName::CLASS_INDEX | AstExprIndexExpr::CLASS_INDEX => true,
      _ => false,
    }
  }
}
