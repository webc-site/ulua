use ulua_ast::records::{ast_expr_local::AstExprLocal, ast_visitor::AstVisitor};

use crate::records::compiler::Compiler;

/// cpp `ConflictVisitor`：登记重复赋值的局部寄存器冲突。
/// `compiler` 借用为只读（查询 `get_local_reg`），取代裸指针自引用。
#[derive(Debug)]
pub struct Visitor<'a> {
  pub(crate) compiler: &'a Compiler,
  pub(crate) conflict: [u64; 4],
  pub(crate) assigned: [u64; 4],
}

impl AstVisitor for Visitor<'_> {
  fn visit_expr_local(&mut self, node: &mut AstExprLocal) -> bool {
    let reg = self.compiler.get_local_reg(node.local);
    if reg >= 0 {
      let idx = reg as usize / 64;
      let bit = 1 << (reg as usize % 64);
      if (self.assigned[idx] & bit) != 0 {
        self.conflict[idx] |= bit;
      }
    }
    true
  }
}

impl<'a> Visitor<'a> {
  /// 构造：登记位图清零。原 `Compiler::visitor_visitor` 样板转发（cpp
  /// 构造器照抄出的重复名）收敛到被构造类型自身。
  pub(crate) fn new(compiler: &'a Compiler) -> Self {
    Self {
      compiler,
      conflict: [0; 4],
      assigned: [0; 4],
    }
  }
}
