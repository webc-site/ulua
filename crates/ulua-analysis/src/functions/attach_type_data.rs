use alloc::sync::Arc;
/// C++ `TypeAttacher : public AstVisitor`. The five overridden `visit`
/// overloads are landed as inherent methods on `TypeAttacher`; this impl
/// bridges them to the `AstVisitor` trait dispatch used by `root->visit(&ta)`.
use core::ffi::c_void;

use ulua_ast::{
  records::{
    allocator::Allocator, ast_expr_function::AstExprFunction, ast_expr_local::AstExprLocal,
    ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn, ast_stat_local::AstStatLocal,
    ast_visitor::AstVisitor,
  },
  visit::AstVisitable,
};

use crate::records::{module::Module, source_module::SourceModule, type_attacher::TypeAttacher};
impl AstVisitor for TypeAttacher {
  fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_local(node as *mut AstStatLocal)
  }

  fn visit_expr_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_local(node as *mut AstExprLocal)
  }

  fn visit_stat_for(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_for(node as *mut AstStatFor)
  }

  fn visit_stat_for_in(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_for_in(node as *mut AstStatForIn)
  }

  fn visit_expr_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_function(node as *mut AstExprFunction)
  }
}

pub fn attach_type_data(source: &mut SourceModule, result: &mut Module) {
  // C++ `TypeAttacher ta(result, source.allocator.get()); source.root->visit(&ta);`
  let mut ta = TypeAttacher::type_attacher_type_attacher(
    result as *mut Module,
    Arc::as_ptr(&source.allocator) as *mut Allocator,
  );
  unsafe {
    (*source.root).visit(&mut ta);
  }
}
