use crate::{
  records::{
    ast_expr_if_else::AstExprIfElse, cst_expr_if_else::CstExprIfElse, printer::Printer,
    writer::Writer,
  },
  rtti::ast_node_try_as,
};

impl<'a, W: Writer> Printer<'a, W> {
  /// cpp `Printer::visit(AstExprIfElse* elseif)`：elseif 链递归展开时子节点
  /// 同样按共享借用读取（打印器只写 `Writer`）。
  pub fn visualize_else_if_expr(&mut self, elseif: &AstExprIfElse) {
    let cst_node = self.lookup_cst_node::<CstExprIfElse>(&elseif.base.base);

    self.visualize_ast_expr(unsafe { &*elseif.condition });

    if let Some(cst_node) = cst_node {
      self.maybe_advance_and_write(&cst_node.then_position, "then", false);
    } else {
      self.writer.keyword("then");
    }

    self.visualize_ast_expr(unsafe { &*elseif.true_expr });

    if elseif.has_else {
      if let Some(cst_node) = cst_node {
        self.advance(cst_node.else_position);
      }

      // elseif 链：else 分支本身仍是 IfElse 节点时递归展开（CST 以 is_else_if 标记）
      let elseifelseif = unsafe { elseif.false_expr.as_ref() }
        .and_then(|else_expr| ast_node_try_as::<AstExprIfElse>(&else_expr.base))
        .filter(|_| cst_node.is_none_or(|cst| cst.is_else_if));
      if let Some(elseifelseif) = elseifelseif {
        self.writer.keyword("elseif");
        self.visualize_else_if_expr(elseifelseif);
        return;
      }

      self.writer.keyword("else");
      self.visualize_ast_expr(unsafe { &*elseif.false_expr });
    }
  }
}
