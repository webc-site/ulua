use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_table::AstExprTable, ast_stat_block::AstStatBlock,
    ast_stat_repeat::AstStatRepeat, ast_visitor::AstVisitor,
  },
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  macros::lint_stat_process,
  records::{
    lint_context::LintContext, lint_context_handle::LintContextHandle, statement::Statement,
  },
};

#[derive(Debug, Clone)]
pub struct LintMultiLineStatement<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
  pub(crate) stack: Vec<Statement>,
}

impl<'ctx> AstVisitor for LintMultiLineStatement<'ctx> {
  fn visit_expr(&mut self, node: &mut AstExpr) -> bool {
    self.visit_ast_expr(node)
  }

  /// cpp `visit(AstExprTable*)` 返回 `false`：表格字面量整体不参与多行检查。
  fn visit_expr_table(&mut self, _node: &mut AstExprTable) -> bool {
    false
  }

  fn visit_stat_repeat(&mut self, node: &mut AstStatRepeat) -> bool {
    self.visit_ast_stat_repeat(node);

    false
  }

  fn visit_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    self.visit_ast_stat_block(node);

    false
  }
}

// —— 原 methods/lint_multi_line_statement_process.rs ——
impl<'ctx> LintMultiLineStatement<'ctx> {
  lint_stat_process!(LintMultiLineStatement { stack: Vec::new() });
}

// —— 原 methods/lint_multi_line_statement_visit_linter.rs ——
impl<'ctx> LintMultiLineStatement<'ctx> {
  /// cpp `visit(AstExpr*)`：`node` 为分析期存活、由 arena 持有的表达式节点共享借用
  /// （cpp 裸指针形参的 Rust 对应），本方法仅读取其 `base.location`。
  pub fn visit_ast_expr(&mut self, node: &AstExpr) -> bool {
    // 不变式：表达式节点必出现在某块遍历过程中，块访问入口已 push 一条
    // Statement，栈非空由该推进顺序蕴含（cpp `stateStack.back()` 同前提）。
    let top = self
      .stack
      .last_mut()
      .expect("块访问入口先 push，表达式期栈恒非空");
    if !top.flagged {
      let location = node.base.location;
      if location.begin.line > top.last_line() {
        top.last_line = location.begin.line;
        if location.begin.column <= top.start.begin.column {
          emit_warning(
            self.context.get(),
            Code::MultiLineStatement,
            location,
            format_args!("Statement spans multiple lines; use indentation to silence"),
          );
          top.flagged = true;
        }
      }
    }
    true
  }
  pub(crate) fn visit_ast_stat_repeat(&mut self, node: &mut AstStatRepeat) -> bool {
    // body 已句柄化为 Node（parser 非空由类型层承载），as_ptr 桥交仍以指针
    // 形态消费的 visit_ast_stat_block。
    self.visit_ast_stat_block(node.body.as_ptr());
    false
  }
  pub(crate) fn visit_ast_stat_block(&mut self, node: *mut AstStatBlock) -> bool {
    let node_ref = unsafe { &*node };
    for stmt in node_ref.body.iter_nodes() {
      let stmt_ref = stmt.get();
      let s = Statement {
        start: stmt_ref.base.location,
        last_line: stmt_ref.base.location.begin.line,
        flagged: false,
      };
      self.stack.push(s);
      unsafe {
        ast_stat_visit(stmt.as_ptr(), self);
      }
      self.stack.pop();
    }
    false
  }
}
