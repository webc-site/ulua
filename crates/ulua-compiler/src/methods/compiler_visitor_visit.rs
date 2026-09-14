use ulua_ast::records::ast_expr_local::AstExprLocal;

use crate::records::visitor::Visitor;

impl Visitor {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
    unsafe {
      let reg = (*self.self_).get_local_reg((*node).local);
      if reg >= 0 {
        let idx = (reg as usize) / 64;
        let bit = 1 << ((reg as usize) % 64);
        if (self.assigned[idx] & bit) != 0 {
          self.conflict[idx] |= bit;
        }
      }
    }
    true
  }
}
