use core::ptr::{from_ref, null};

use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{function_type::FunctionType, lint_deprecated_api::LintDeprecatedApi},
};
impl LintDeprecatedApi {
  pub fn get_function_type(&self, node: *mut AstExpr) -> *const FunctionType {
    // SAFETY: context 在 linter 存活期内有效；node 由 lint 遍历分发器保证存活。
    let ty = unsafe { (*self.context).get_type(node) };
    ty.and_then(|t| get_type_id::<FunctionType>(follow_type_id(t)))
      .map_or(null(), from_ref)
  }
}
