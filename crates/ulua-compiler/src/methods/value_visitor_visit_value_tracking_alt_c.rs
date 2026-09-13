//! Node: `cxx:Method:Luau.Compiler:Compiler/src/ValueTracking.cpp:61:visit`
//!
//! `ValueVisitor::visit(AstStatCompoundAssign*)` — mark the compound-assignment
//! target written, then recurse into the value expression. Returns false: the
//! children are traversed here, not by the generic walker.

use ulua_ast::{records::ast_stat_compound_assign::AstStatCompoundAssign, visit::ast_expr_visit};

use crate::records::value_visitor::ValueVisitor;

impl ValueVisitor {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_stat_compound_assign(
    &mut self,
    node: *mut AstStatCompoundAssign,
  ) -> bool {
    unsafe {
      let node = &*node;
      self.assign(node.var);
      ast_expr_visit(node.value, self);
    }

    false
  }
}
