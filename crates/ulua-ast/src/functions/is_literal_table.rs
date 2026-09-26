use crate::{
  functions::is_constant_literal::is_constant_literal,
  records::{
    ast_expr::AstExpr,
    ast_expr_table::{AstExprTable, ItemKind},
  },
  rtti::ast_node_try_as,
};

/// cpp `isLiteralTable(const AstExpr* expr)`（`Ast/include/Luau/Ast.h:1735`）。
///
/// 只读判定，故收共享引用：判空下转换成 [`ast_node_try_as`]，
/// 原本手写的 `expr as *mut AstNode` + 两次 null 检查一并消失。
pub fn is_literal_table(expr: &AstExpr) -> bool {
  // 非表节点即 false（cpp 的 `expr->as<AstExprTable>() == nullptr` 分支）
  let Some(table) = ast_node_try_as::<AstExprTable>(&expr.base) else {
    return false;
  };

  table.items.iter().all(|item| match item.kind {
    // cpp: general（`[k]=v`）形式不算字面量表
    ItemKind::General => false,
    ItemKind::Record | ItemKind::List => {
      // Safety: 记录/列表项的 value 由 parser 写入 arena，恒非空且存活
      //（cpp 侧同一前提下直接 `isConstantLiteral(item.value)`）
      let value = unsafe { &*item.value };
      is_constant_literal(value) || is_literal_table(value)
    }
  })
}
