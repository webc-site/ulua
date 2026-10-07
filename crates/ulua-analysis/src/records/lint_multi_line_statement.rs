use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_table::AstExprTable, ast_stat_block::AstStatBlock,
    ast_stat_repeat::AstStatRepeat, ast_visitor::AstVisitor,
  },
  visit::ast_stat_visit_ref,
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
  /// cpp `visit(AstExpr*)`：原 inherent `visit_ast_expr` 实体并入本覆写（消除
  /// 同文件转发 shim）；`node` 为分析期存活、由 arena 持有的表达式节点借用
  /// （cpp 裸指针形参的 Rust 对应），本覆写仅读取其 `base.location`。
  fn visit_expr(&mut self, node: &mut AstExpr) -> bool {
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

// —— 原 methods/lint_multi_line_statement_visit_linter.rs（visit_ast_expr 实体
// 已并入上方 AstVisitor::visit_expr 覆写） ——
impl<'ctx> LintMultiLineStatement<'ctx> {
  pub(crate) fn visit_ast_stat_repeat(&mut self, node: &mut AstStatRepeat) -> bool {
    // body 已句柄化为 Node（parser 非空由类型层承载）：`get_mut()` 物化独占借用，
    // visit_ast_stat_block 随之收窄为引用形参，`as_ptr` 裸指针桥退役。
    self.visit_ast_stat_block(node.body.get_mut());
    false
  }
  pub(crate) fn visit_ast_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    // body 槽位经 `iter_nodes_mut` 沿 `&mut self` 传递独占，逐槽 `get_mut()` 交出
    // `&mut AstStat` 喂 `ast_stat_visit_ref` 引用门面（同 cpp `stmt->visit(this)`），
    // 全链路无裸指针，本函数不再需要 `unsafe`。
    for stmt in node.body.iter_nodes_mut() {
      let stmt_ref = stmt.get();
      let s = Statement {
        start: stmt_ref.base.location,
        last_line: stmt_ref.base.location.begin.line,
        flagged: false,
      };
      self.stack.push(s);
      ast_stat_visit_ref(stmt.get_mut(), self);
      self.stack.pop();
    }
    false
  }
}
