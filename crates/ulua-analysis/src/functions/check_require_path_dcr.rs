use core::{ffi::CStr, ptr::NonNull};

use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_index_name::AstExprIndexName, ast_node::AstNode},
  rtti::ast_node_as,
};

use crate::{
  records::{constraint_solver::ConstraintSolver, deprecated_api_used::DeprecatedApiUsed},
  type_aliases::type_error_data::TypeErrorData,
};
pub fn check_require_path_dcr(mut solver: NonNull<ConstraintSolver>, expr: *mut AstExpr) -> bool {
  let mut good = true;
  let mut index_expr = unsafe { ast_node_as::<AstExprIndexName>(expr as *mut AstNode) };

  while !index_expr.is_null() {
    let index_ref = unsafe { &*index_expr };
    let index_bytes = unsafe { CStr::from_ptr(index_ref.index.value).to_bytes() };

    if index_bytes == b"parent" {
      let deprecated = DeprecatedApiUsed {
        symbol: "parent".to_string(),
        use_instead: "Parent".to_string(),
      };
      let data = TypeErrorData::DeprecatedApiUsed(deprecated);
      unsafe {
        solver
          .as_mut()
          .report_error_type_error_data_location(data, &index_ref.index_location);
      }
      good = false;
    }

    index_expr = unsafe { ast_node_as::<AstExprIndexName>(index_ref.expr as *mut AstNode) };
  }

  good
}
