use core::ffi::c_void;

use crate::{
  records::{ast_expr_function::AstExprFunction, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_stat_visit, ast_type_pack_visit, ast_type_visit},
};

impl AstVisitable for AstExprFunction {
  /// cpp `AstExprFunction::visit(AstVisitor*)`（`Ast/src/Ast.cpp:336`）：`this`
  /// 是非 const 指针，visitor（如 Analysis 的 TypeAttacher::attachTypes）会在
  /// dispatch 期间写穿本节点回填 `returnAnnotation`。本 trait 因此取
  /// `&mut self`（见 `visit.rs` 的总说明），交出的 `*mut Self` 由独占借用派生，
  /// 写穿合法——无需再以 `black_box` 遮掩 `&self` 的 readonly 属性。
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    if visitor.visit_expr_function(self as *mut Self as *mut c_void) {
      for arg_ptr in self.args.as_slice() {
        let arg = unsafe { &*(*arg_ptr) };
        if !arg.annotation.is_null() {
          unsafe {
            ast_type_visit(arg.annotation, visitor);
          }
        }
      }

      if !self.vararg_annotation.is_null() {
        unsafe {
          ast_type_pack_visit(self.vararg_annotation, visitor);
        }
      }

      if !self.return_annotation.is_null() {
        unsafe {
          ast_type_pack_visit(self.return_annotation, visitor);
        }
      }

      unsafe {
        ast_stat_visit(self.body as *mut _, visitor);
      }
    }
  }
}
