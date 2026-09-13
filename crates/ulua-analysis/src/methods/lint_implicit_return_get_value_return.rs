use core::{ffi::c_void, ptr::null_mut};

use ulua_ast::{
  records::{ast_stat::AstStat, ast_stat_return::AstStatReturn, ast_visitor::AstVisitor},
  visit::ast_stat_visit,
};

use crate::records::lint_implicit_return::LintImplicitReturn;
pub fn lint_implicit_return_get_value_return(
  _this: &mut LintImplicitReturn,
  node: *mut c_void,
) -> *mut AstStatReturn {
  struct Visitor {
    result: *mut AstStatReturn,
  }

  impl AstVisitor for Visitor {
    fn visit_expr(&mut self, _node: *mut c_void) -> bool {
      false
    }

    fn visit_stat_return(&mut self, node: *mut c_void) -> bool {
      let node = node as *mut AstStatReturn;
      unsafe {
        if self.result.is_null() && (*node).list.size > 0 {
          self.result = node;
        }
      }

      false
    }
  }

  let mut visitor = Visitor { result: null_mut() };
  unsafe {
    ast_stat_visit(node as *mut AstStat, &mut visitor);
  }
  visitor.result
}
