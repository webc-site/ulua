use core::ffi::c_void;

use ulua_ast::{
  records::{
    ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_local::AstExprLocal, ast_stat_assign::AstStatAssign, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_type::AstType, ast_type_pack::AstTypePack,
    ast_type_reference::AstTypeReference, ast_visitor::AstVisitor,
  },
  visit::ast_stat_visit,
};

use crate::records::{
  global_linter_alt_c::Global, lint_context::LintContext, lint_local_hygiene::LintLocalHygiene,
};
impl AstVisitor for LintLocalHygiene {
  fn visit_stat_assign(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_assign(node as *mut AstStatAssign)
  }

  fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_local(node as *mut AstStatLocal)
  }

  fn visit_stat_local_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_local_function(node as *mut AstStatLocalFunction)
  }

  fn visit_expr_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_local(node as *mut AstExprLocal)
  }

  fn visit_expr_global(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_global(node as *mut AstExprGlobal)
  }

  fn visit_expr_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_function(node as *mut AstExprFunction)
  }

  fn visit_type(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type(node as *mut AstType)
  }

  fn visit_type_reference(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_reference(node as *mut AstTypeReference)
  }

  fn visit_type_pack(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_pack(node as *mut AstTypePack)
  }
}

pub fn lint_local_hygiene_process(context: &mut LintContext) {
  let mut pass = LintLocalHygiene::new();
  pass.context = context as *mut LintContext;

  for (global_name, global) in context.builtin_globals.iter() {
    let g = Global {
      builtin: true,
      ..Default::default()
    };
    let _ = global;
    pass.globals.try_insert(*global_name, g);
  }

  unsafe { ast_stat_visit(context.root, &mut pass) };
  pass.report();
}
