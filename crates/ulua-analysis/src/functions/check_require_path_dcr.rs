use core::ptr::NonNull;

use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_index_name::AstExprIndexName},
  rtti::ast_node_try_as_ptr,
};

use crate::{
  records::{constraint_solver::ConstraintSolver, deprecated_api_used::DeprecatedApiUsed},
  type_aliases::type_error_data::TypeErrorData,
};

pub(crate) fn check_require_path_dcr(
  mut solver: NonNull<ConstraintSolver>,
  mut expr: *mut AstExpr,
) -> bool {
  let mut good = true;

  // Safety: `expr` 由 DCR 调用方传入，指向存活 AST 节点或为 null；`ast_node_try_as_ptr`
  // 自带判空与 class_index 判型，命中即合法下转为只读借用，未命中返回 None。
  while let Some(index_ref) = unsafe { ast_node_try_as_ptr::<AstExprIndexName>(expr) } {
    let index_bytes = index_ref.index.as_bytes();

    if index_bytes == b"parent" {
      let deprecated = DeprecatedApiUsed {
        symbol: "parent".to_string(),
        use_instead: "Parent".to_string(),
      };
      let data = TypeErrorData::DeprecatedApiUsed(deprecated);
      // Safety: `solver` 是构造期传入的非空 `NonNull<ConstraintSolver>`，本函数在单线程
      // 串行执行，重建 `&mut` 不产生并发别名；`index_ref` 是另一对象（AST 节点）的共享借用，
      // 与 solver 指向不同内存，二者互不冲突。
      unsafe {
        solver
          .as_mut()
          .report_error_type_error_data_location(data, &index_ref.index_location);
      }
      good = false;
    }

    // expr 已句柄化恒非空；行走链为既有裸指针 API，经 as_ptr 桥接。
    expr = index_ref.expr.as_ptr();
  }

  good
}
