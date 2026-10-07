use alloc::string::ToString;

use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_index_name::AstExprIndexName, node_handle::OptNode},
  rtti::ast_node_try_as,
};

use crate::{
  records::{deprecated_api_used::DeprecatedApiUsed, type_checker::TypeChecker},
  type_aliases::type_error_data::TypeErrorData,
};

pub(crate) fn check_require_path(typechecker: &mut TypeChecker, mut expr: *mut AstExpr) -> bool {
  let mut good = true;

  // 入参仍是指针：每轮先经句柄门面 `OptNode::from_ptr` 把可空性折叠为
  // `Option`，判型下转走生命周期正确的 [`ast_node_try_as`]，借用半径由本轮
  // 局部句柄供给，不再锻造假 `'static`。
  loop {
    let handle = OptNode::from_ptr(expr);
    let Some(expr_ref) = handle.get() else {
      break;
    };
    let Some(index_ref) = ast_node_try_as::<AstExprIndexName>(expr_ref) else {
      break;
    };
    let index_bytes = index_ref.index.as_bytes();

    if index_bytes == b"parent" {
      typechecker.report_error_location_type_error_data(
        &index_ref.index_location,
        TypeErrorData::DeprecatedApiUsed(DeprecatedApiUsed {
          symbol: "parent".to_string(),
          use_instead: "Parent".to_string(),
        }),
      );
      good = false;
    }

    // expr 已句柄化恒非空；行走链为既有裸指针 API，经 as_ptr 桥接。
    expr = index_ref.expr.as_ptr();
  }

  good
}
