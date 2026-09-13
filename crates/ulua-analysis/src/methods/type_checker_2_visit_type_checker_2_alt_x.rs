use ulua_ast::records::{
  ast_class_method::AstClassMethod, ast_class_property::AstClassProperty,
  ast_stat_class::AstStatClass,
};
use ulua_common::{FFlag, LUAU_ASSERT};

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_stat_class(&mut self, stat: *mut AstStatClass) {
    LUAU_ASSERT!(FFlag::DebugLuauUserDefinedClasses.get());

    unsafe {
      let members = &(*stat).members;
      for member in members.as_slice() {
        if let Some(prop) = member.get_if::<AstClassProperty>() {
          if !prop.ty.is_null() {
            self.visit_ast_type(prop.ty);
          }
        } else if let Some(method) = member.get_if::<AstClassMethod>() {
          // SAFETY: function 指向 AST arena 节点。
          self.visit_ast_expr_function(&*method.function);
        } else {
          LUAU_ASSERT!(false);
        }
      }
    }
  }
}
