use ulua_ast::records::ast_stat_type_function::AstStatTypeFunction;

use crate::records::function_visitor::FunctionVisitor;

impl<'a> FunctionVisitor<'a> {
  pub fn visit_ast_stat_type_function(&mut self, _node: *mut AstStatTypeFunction) -> bool {
    false
  }
}
