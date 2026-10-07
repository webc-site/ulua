use ulua_ast::records::{ast_expr_global::AstExprGlobal, ast_visitor::AstVisitor};

/// cpp `FenvVisitor`：探测顶层是否用到 getfenv/setfenv。
/// cpp 以两个 `bool&` 出参引用字段直写 Compiler 成员；Rust 侧折叠为
/// 自有所有权字段，遍历结束后由调用方一次性回写（出参→返回值惯例）。
#[derive(Debug, Default)]
pub(crate) struct FenvVisitor {
  pub(crate) getfenv_used: bool,
  pub(crate) setfenv_used: bool,
}

impl AstVisitor for FenvVisitor {
  fn visit_expr_global(&mut self, node: &mut AstExprGlobal) -> bool {
    self.visit(node)
  }
}

impl FenvVisitor {
  /// 对应 cpp `FenvVisitor::visit(AstExprGlobal*)`：命中即置自有标志位。
  pub(crate) fn visit(&mut self, node: &AstExprGlobal) -> bool {
    let name_bytes = node.name.as_bytes();
    if name_bytes == b"getfenv" {
      self.getfenv_used = true;
    }
    if name_bytes == b"setfenv" {
      self.setfenv_used = true;
    }
    false
  }
}
