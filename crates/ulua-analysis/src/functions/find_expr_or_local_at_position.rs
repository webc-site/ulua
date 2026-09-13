use core::ffi::c_void;

use ulua_ast::records::{ast_visitor::AstVisitor, position::Position};

use crate::records::{
  expr_or_local::ExprOrLocal, find_expr_or_local::FindExprOrLocal, source_module::SourceModule,
};
pub fn find_expr_or_local_at_position(source: &SourceModule, pos: Position) -> ExprOrLocal {
  let mut find_visitor = FindExprOrLocal::new(pos);
  find_visitor.visit_stat_block(source.root as *mut c_void);
  find_visitor.result
}
