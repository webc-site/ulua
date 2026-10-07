use core::ptr::NonNull;

use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_index_name::AstExprIndexName, node_handle::OptNode},
  rtti::ast_node_try_as,
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
