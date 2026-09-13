use crate::{
  records::{
    ast_expr_if_else::AstExprIfElse, ast_node::AstNode, cst_expr_if_else::CstExprIfElse,
    printer::Printer,
  },
  rtti::ast_node_as,
};

impl<'a> Printer<'a> {
  pub fn visualize_else_if_expr(&mut self, elseif: &mut AstExprIfElse) {
    let cst_node =
      self.lookup_cst_node::<CstExprIfElse>(elseif as *mut AstExprIfElse as *mut AstNode);

    self.visualize_ast_expr(unsafe { &mut *elseif.condition });

    if !cst_node.is_null() {
      unsafe {
        self.maybe_advance_and_write(&(*cst_node).then_position, "then", false);
      }
    } else {
      self.writer.keyword("then");
    }

    self.visualize_ast_expr(unsafe { &mut *elseif.true_expr });

    if elseif.has_else {
      if !cst_node.is_null() {
        unsafe { self.advance((*cst_node).else_position) };
      }

      let else_expr = elseif.false_expr as *mut AstExprIfElse;
      if !else_expr.is_null() {
        let elseifelseif = unsafe { ast_node_as::<AstExprIfElse>(else_expr as *mut AstNode) };
        if !elseifelseif.is_null() && (cst_node.is_null() || unsafe { (*cst_node).is_else_if }) {
          self.writer.keyword("elseif");
          self.visualize_else_if_expr(unsafe { &mut *elseifelseif });
          return;
        }
      }

      self.writer.keyword("else");
      self.visualize_ast_expr(unsafe { &mut *elseif.false_expr });
    }
  }
}
