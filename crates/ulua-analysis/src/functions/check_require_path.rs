use alloc::string::ToString;

use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_index_name::AstExprIndexName},
  rtti::ast_node_try_as_ptr,
};

use crate::{
  records::{deprecated_api_used::DeprecatedApiUsed, type_checker::TypeChecker},
  type_aliases::type_error_data::TypeErrorData,
};

pub(crate) fn check_require_path(typechecker: &mut TypeChecker, mut expr: *mut AstExpr) -> bool {
  let mut good = true;

  // Safety: `expr` 由调用方从 parser arena 的存活 ASTExpr 节点传入，或为 null；
  // `ast_node_try_as_ptr` 自带判空与 class_index 判型，命中即合法下转为只读借用，未命中返回 None。
  while let Some(index_ref) = unsafe { ast_node_try_as_ptr::<AstExprIndexName>(expr) } {
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
