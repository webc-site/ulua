//! Node: `cxx:Method:Luau.Compiler:Compiler/src/ValueTracking.cpp:50:visit`
//!
//! `ValueVisitor::visit(AstStatAssign*)` — mark every assignment target written,
//! then recurse into the value expressions (so nested assignments like
//! `t[function() t = nil end] = 5` are tracked). Returns false: traversal of the
//! children is performed here, not by the generic walker.

use ulua_ast::{records::ast_stat_assign::AstStatAssign, visit::ast_expr_visit};

use crate::records::value_visitor::ValueVisitor;

impl ValueVisitor {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_stat_assign(&mut self, node: *mut AstStatAssign) -> bool {
    unsafe {
      let node = &*node;
      for &var in node.vars.iter() {
        self.assign(var);
      }
      for &value in node.values.iter() {
        ast_expr_visit(value, self);
      }
    }

    false
  }
}
