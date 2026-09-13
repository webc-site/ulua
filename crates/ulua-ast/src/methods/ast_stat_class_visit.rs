use core::ffi::c_void;

use ulua_common::{FFlag::DebugLuauUserDefinedClasses, records::variant::Variant2};

use crate::{
  records::{
    ast_class_method::AstClassMethod, ast_class_property::AstClassProperty,
    ast_stat_class::AstStatClass, ast_visitor::AstVisitor,
  },
  visit::{AstVisitable, ast_expr_visit, ast_type_visit},
};

impl AstVisitable for AstStatClass {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    ulua_common::LUAU_ASSERT!(DebugLuauUserDefinedClasses.get());

    if visitor.visit_stat_class(self as *const Self as *mut c_void) {
      if !self.super_.is_null() {
        unsafe {
          ast_expr_visit(self.super_, visitor);
        }
      }

      for member in self.members.iter() {
        match member {
          Variant2::V0(prop) => {
            let prop: &AstClassProperty = prop;
            if !prop.ty.is_null() {
              unsafe {
                ast_type_visit(prop.ty, visitor);
              }
            }
          }
          Variant2::V1(method) => {
            let method: &AstClassMethod = method;
            unsafe {
              ast_expr_visit(method.function as *mut _, visitor);
            }
          }
        }
      }
    }
  }
}

pub fn ast_stat_class_visit(this: &AstStatClass, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
